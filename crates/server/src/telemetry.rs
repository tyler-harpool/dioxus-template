use axum::{body::Body, http::Request, response::Response};
use opentelemetry::{
    global,
    trace::{SpanKind, TraceContextExt, Tracer},
    Context, KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use std::{
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use tower::{Layer, Service};

/// Set up the OpenTelemetry TracerProvider and register it globally.
///
/// Must be called inside a Tokio runtime (the batch exporter spawns a
/// background flush task). Reads config from environment:
///   - `OTEL_EXPORTER_OTLP_ENDPOINT` — collector gRPC address (e.g. `http://localhost:4317`)
///   - `OTEL_SERVICE_NAME` — service name tag (default: `dioxus-app`)
pub fn init_telemetry() {
    let _ = dotenvy::dotenv();

    let endpoint = match std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        Ok(ep) => ep,
        Err(_) => {
            eprintln!("OTEL_EXPORTER_OTLP_ENDPOINT not set, skipping OTLP telemetry");
            return;
        }
    };

    let service_name =
        std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "dioxus-app".to_string());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(&endpoint)
        .build()
        .expect("Failed to create OTLP exporter");

    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, runtime::Tokio)
        .with_resource(opentelemetry_sdk::Resource::new(vec![KeyValue::new(
            "service.name",
            service_name,
        )]))
        .build();

    global::set_tracer_provider(provider);

    eprintln!("Telemetry initialized — exporting to {endpoint}");
}

/// Tower layer that creates an OpenTelemetry span for each HTTP request.
#[derive(Clone)]
pub struct OtelTraceLayer;

impl<S> Layer<S> for OtelTraceLayer {
    type Service = OtelTraceService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OtelTraceService { inner }
    }
}

#[derive(Clone)]
pub struct OtelTraceService<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for OtelTraceService<S>
where
    S: Service<Request<Body>, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let tracer = global::tracer("dioxus-app");
        let method = req.method().to_string();
        let path = req.uri().path().to_string();

        let span = tracer
            .span_builder(format!("{method} {path}"))
            .with_kind(SpanKind::Server)
            .with_attributes(vec![
                KeyValue::new("http.method", method),
                KeyValue::new("http.target", path),
            ])
            .start(&tracer);

        let cx = Context::current_with_span(span);
        let mut inner = self.inner.clone();

        // Attach the OTel context, get the response future, then drop the guard
        // before awaiting (ContextGuard is not Send).
        let guard = cx.clone().attach();
        let future = inner.call(req);
        drop(guard);

        Box::pin(async move {
            let response = future.await?;

            let span = cx.span();
            span.set_attribute(KeyValue::new(
                "http.status_code",
                response.status().as_u16() as i64,
            ));
            if response.status().is_server_error() {
                span.set_status(opentelemetry::trace::Status::error(
                    response.status().to_string(),
                ));
            }

            Ok(response)
        })
    }
}
