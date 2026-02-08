use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Categorization of application errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum AppErrorKind {
    NotFound,
    ValidationError,
    DatabaseError,
    Unauthorized,
    Forbidden,
    InternalError,
}

impl fmt::Display for AppErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErrorKind::NotFound => write!(f, "NotFound"),
            AppErrorKind::ValidationError => write!(f, "ValidationError"),
            AppErrorKind::DatabaseError => write!(f, "DatabaseError"),
            AppErrorKind::Unauthorized => write!(f, "Unauthorized"),
            AppErrorKind::Forbidden => write!(f, "Forbidden"),
            AppErrorKind::InternalError => write!(f, "InternalError"),
        }
    }
}

/// Structured application error used across server and client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AppError {
    pub kind: AppErrorKind,
    pub message: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub field_errors: HashMap<String, String>,
}

impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            kind: AppErrorKind::NotFound,
            message: message.into(),
            field_errors: HashMap::new(),
        }
    }

    pub fn validation(message: impl Into<String>, field_errors: HashMap<String, String>) -> Self {
        Self {
            kind: AppErrorKind::ValidationError,
            message: message.into(),
            field_errors,
        }
    }

    pub fn database(message: impl Into<String>) -> Self {
        Self {
            kind: AppErrorKind::DatabaseError,
            message: message.into(),
            field_errors: HashMap::new(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            kind: AppErrorKind::Unauthorized,
            message: message.into(),
            field_errors: HashMap::new(),
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            kind: AppErrorKind::Forbidden,
            message: message.into(),
            field_errors: HashMap::new(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            kind: AppErrorKind::InternalError,
            message: message.into(),
            field_errors: HashMap::new(),
        }
    }

    /// Parse an AppError from a ServerFnError message string (client-side).
    ///
    /// `ServerFnError::to_string()` wraps the payload like:
    ///   `error running server function: {"kind":"Unauthorized",...} (details: None)`
    /// This method extracts the embedded JSON and parses it.
    pub fn from_server_error(error_message: &str) -> Option<Self> {
        // Try direct parse first (in case the string is raw JSON)
        if let Ok(err) = serde_json::from_str::<Self>(error_message) {
            return Some(err);
        }
        // Extract the JSON object embedded between the first `{` and last `}`
        let start = error_message.find('{')?;
        let end = error_message.rfind('}')?;
        if end > start {
            serde_json::from_str(&error_message[start..=end]).ok()
        } else {
            None
        }
    }

    /// Extract a user-friendly error message from a `ServerFnError.to_string()`.
    ///
    /// Parses the embedded `AppError` JSON and returns its `message` field.
    /// Falls back to a generic message if parsing fails.
    pub fn friendly_message(error_string: &str) -> String {
        if let Some(app_error) = Self::from_server_error(error_string) {
            app_error.message
        } else {
            "Something went wrong. Please try again.".to_string()
        }
    }

    #[cfg_attr(not(feature = "server"), allow(dead_code))]
    fn status_code_u16(&self) -> u16 {
        match self.kind {
            AppErrorKind::NotFound => 404,
            AppErrorKind::ValidationError => 422,
            AppErrorKind::DatabaseError => 500,
            AppErrorKind::Unauthorized => 401,
            AppErrorKind::Forbidden => 403,
            AppErrorKind::InternalError => 500,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for AppError {}

#[cfg(feature = "validation")]
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut field_errors = HashMap::new();
        for (field, errs) in errors.field_errors() {
            if let Some(first) = errs.first() {
                let msg = first
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid value for {}", field));
                field_errors.insert(field.to_string(), msg);
            }
        }
        AppError::validation("Validation failed", field_errors)
    }
}

#[cfg(feature = "server")]
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = axum::http::StatusCode::from_u16(self.status_code_u16())
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(self)).into_response()
    }
}
