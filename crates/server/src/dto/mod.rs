use openapi::model::error::ApiErrorContext;

pub mod error;
pub mod tag;

pub(crate) trait MapToApiModel<T> {
    fn map_to_api_model(self) -> T;
}

pub trait MapToApiError {
    fn map_to_api_error(&self) -> ApiErrorContext;
}

pub trait MapToDomain<T> {
    fn map_to_domain(self) -> T;
}

#[macro_export]
macro_rules! map_to_api_response {
    ($result:expr, $map_to_response_function:ident) => {
        $map_to_response_function(
            $result
                .map(|value| value.map_to_api_model())
                .map_err(|error| error.map_to_api_error()),
        )
    };
}

#[macro_export]
macro_rules! map_list_to_api_response {
    ($result:expr, $map_to_response_function:ident) => {
        $map_to_response_function(
            $result
                .map(|values| {
                    values
                        .into_iter()
                        .map(|value| value.map_to_api_model())
                        .collect::<Vec<_>>()
                })
                .map_err(|error| error.map_to_api_error()),
        )
    };
}
