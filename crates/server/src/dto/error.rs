use crate::dto::MapToApiError;
use domain::Error;
use openapi::model::error::{ApiErrorContext, ApiErrorType};

impl MapToApiError for Error {
    fn map_to_api_error(&self) -> ApiErrorContext {
        ApiErrorContext {
            error_type: map_to_api_error_type(&self),
            message: "".to_string(),
            errors: vec![],
        }
    }
}

fn map_to_api_error_type(error: &Error) -> ApiErrorType {
    match error {
        Error::BadRequest => ApiErrorType::BadRequest,
        Error::Unauthorized => ApiErrorType::Unauthorized,
        Error::Forbidden => ApiErrorType::Forbidden,
        Error::NotFound => ApiErrorType::NotFound,
        Error::InternalServerError => ApiErrorType::InternalServerError,
        Error::TooManyRequests => ApiErrorType::TooManyRequests,
        Error::ServiseMaintenance => ApiErrorType::ServiseMaintenance,
    }
}
