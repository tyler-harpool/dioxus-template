use axum::{body::Body, http::Request, response::Response};
use opentelemetry::{
    global,
    trace::{SpanKind, TraceContextExt, Tracer},
    Context, KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use std::{
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use tower::{Layer, Service};

use crate::auth::jwt::Claims;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    let environment = std::env::var("DEPLOY_ENV").unwrap_or_else(|_| "development".to_string());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(&endpoint)
        .build()
        .expect("Failed to create OTLP exporter");

    let resource = opentelemetry_sdk::Resource::builder()
        .with_service_name(service_name)
        .with_attribute(KeyValue::new("service.version", APP_VERSION))
        .with_attribute(KeyValue::new("deployment.environment", environment))
        .build();

    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    global::set_tracer_provider(provider);

    eprintln!("Telemetry initialized v{APP_VERSION} — exporting to {endpoint}");
}

/// Detect client platform from User-Agent and optional X-Client-Platform header.
///
/// Priority: explicit `X-Client-Platform` header > User-Agent heuristic.
/// Dioxus native clients (desktop/mobile) don't send User-Agent, so they
/// show as "native" unless the app sets X-Client-Platform.
fn detect_platform(ua: &str, explicit: Option<&str>) -> &'static str {
    // Honour explicit header first (set by custom Dioxus client middleware)
    if let Some(p) = explicit {
        return match p {
            "ios" => "ios",
            "android" => "android",
            "desktop" => "desktop",
            "mobile" => "mobile",
            "web" => "web",
            _ => "unknown",
        };
    }

    // Heuristic from User-Agent
    if ua == "unknown" || ua.is_empty() {
        // No UA → native Dioxus client (desktop or mobile)
        return "native";
    }
    if ua.contains("iPhone") || ua.contains("iPad") || ua.contains("CFNetwork") {
        "ios"
    } else if ua.contains("Android") {
        "android"
    } else if ua.contains("Mozilla") || ua.contains("Chrome") || ua.contains("Safari") {
        "web"
    } else {
        "native"
    }
}

/// Tower layer that creates an OpenTelemetry span for each HTTP request.
///
/// Captures: method, path, user-agent, client platform, request ID,
/// response status, and authenticated user info (if present).
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

        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        let explicit_platform = req
            .headers()
            .get("x-client-platform")
            .and_then(|v| v.to_str().ok());
        let client_platform = detect_platform(&user_agent, explicit_platform);

        let request_id = req
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        // Extract auth claims if the auth middleware already ran
        let auth_attrs: Vec<KeyValue> = if let Some(claims) = req.extensions().get::<Claims>() {
            vec![
                KeyValue::new("user.id", claims.sub),
                KeyValue::new("user.email", claims.email.clone()),
                KeyValue::new("user.role", claims.role.clone()),
                KeyValue::new("user.tier", claims.tier.clone()),
                KeyValue::new("auth.status", "authenticated"),
            ]
        } else {
            vec![KeyValue::new("auth.status", "anonymous")]
        };

        let mut attributes = vec![
            KeyValue::new("http.method", method.clone()),
            KeyValue::new("http.target", path.clone()),
            KeyValue::new("http.user_agent", user_agent),
            KeyValue::new("client.platform", client_platform),
            KeyValue::new("http.request_id", request_id),
        ];
        attributes.extend(auth_attrs);

        // Use path as the route name (strip hashes for server fn endpoints)
        let route = path
            .trim_end_matches(|c: char| c.is_ascii_digit())
            .to_string();

        let span = tracer
            .span_builder(format!("{} {}", &method, &route))
            .with_kind(SpanKind::Server)
            .with_attributes(attributes)
            .start(&tracer);

        let cx = Context::current_with_span(span);
        let mut inner = self.inner.clone();

        let guard = cx.clone().attach();
        let future = inner.call(req);
        drop(guard);

        Box::pin(async move {
            let response = future.await?;

            let span = cx.span();
            let status = response.status();
            span.set_attribute(KeyValue::new("http.status_code", status.as_u16() as i64));

            if status.is_server_error() {
                span.set_status(opentelemetry::trace::Status::error(status.to_string()));
            } else if status.is_client_error() {
                span.set_attribute(KeyValue::new("error.type", "client_error"));
            }

            Ok(response)
        })
    }
}
