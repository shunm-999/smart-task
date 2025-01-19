use domain::Error;
use openapi::model::error::{ApiErrorType, ErrorContext};

pub struct ErrorResponse {
    error: Error,
    message: String,
    errors: Vec<String>,
}

impl ErrorResponse {
    pub fn from_error(error: Error) -> Self {
        ErrorResponse {
            error,
            message: "".to_string(),
            errors: vec![],
        }
    }
}

impl Into<ErrorContext> for ErrorResponse {
    fn into(self) -> ErrorContext {
        let message = self.message;
        let errors = self.errors;
        match self.error {
            Error::BadRequest => ErrorContext {
                error_type: ApiErrorType::BadRequest,
                message,
                errors,
            },
            Error::Unauthorized => ErrorContext {
                error_type: ApiErrorType::Unauthorized,
                message,
                errors,
            },
            Error::Forbidden => ErrorContext {
                error_type: ApiErrorType::Forbidden,
                message,
                errors,
            },
            Error::NotFound => ErrorContext {
                error_type: ApiErrorType::NotFound,
                message,
                errors,
            },
            Error::InternalServerError => ErrorContext {
                error_type: ApiErrorType::InternalServerError,
                message,
                errors,
            },
            Error::TooManyRequests => ErrorContext {
                error_type: ApiErrorType::TooManyRequests,
                message,
                errors,
            },
            Error::NotDeletableResource => ErrorContext {
                error_type: ApiErrorType::NotDeletableResource,
                message,
                errors,
            },
            Error::ServiseMaintenance => ErrorContext {
                error_type: ApiErrorType::ServiseMaintenance,
                message,
                errors,
            },
        }
    }
}
