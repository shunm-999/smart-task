use smart_task_openapi_axum::models::{ModelsPeriodErrorCode, ModelsPeriodErrorType};

pub(crate) type ApiErrorCode = ModelsPeriodErrorCode;

pub type ApiErrorType = ModelsPeriodErrorType;

pub struct ErrorContext {
    pub error_type: ApiErrorType,
    pub message: String,
    pub errors: Vec<String>,
}
