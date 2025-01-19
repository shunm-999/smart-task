pub mod project;
pub mod tag;
pub mod task;

#[macro_export]
macro_rules! create_response {
    ($name:ident, $inner:ident, $map_trait:ident, $entity:ty, $map_function:ident) => {
        crate::response_definition!(
            $name,
            $inner,
            $map_trait,
            $entity,
            $map_function,
            Status201_TheRequestHasSucceededAndANewResourceHasBeenCreatedAsAResult,
            true
        );
    };
}

#[macro_export]
macro_rules! ok_response {
    ($name:ident, $inner:ident, $map_trait:ident, $entity:ty, $map_function:ident) => {
        crate::response_definition!(
            $name,
            $inner,
            $map_trait,
            $entity,
            $map_function,
            Status200_TheRequestHasSucceeded,
            false
        );
    };
}

#[macro_export]
macro_rules! response_definition {
    ($name:ident, $inner:ident, $map_trait:ident, $entity:ty, $map_function:ident, $success_status_code:ident, $is_create_response:ident) => {
        // 動的に構造体を定義
            struct $name($inner);

            // From トレイトの実装
            impl From<$name> for actix_web::HttpResponse {
                fn from(value: $name) -> Self {
                    let value = value.0;

                    match value {
                        $inner::$success_status_code(res) => {
                          if($is_create_response) {
                            actix_web::HttpResponse::Created().json(res)
                          } else {
                            actix_web::HttpResponse::Ok().json(res)
                          }
                        }
                        $inner::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            actix_web::HttpResponse::BadRequest().json(res)
                        }
                        $inner::Status401_AccessIsUnauthorized(res) => {
                            actix_web::HttpResponse::Unauthorized().json(res)
                        }
                        $inner::Status403_AccessIsForbidden(res) => {
                            actix_web::HttpResponse::Forbidden().json(res)
                        }
                        $inner::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            actix_web::HttpResponse::NotFound().json(res)
                        }
                        $inner::Status429_ClientError(res) => {
                            actix_web::HttpResponse::TooManyRequests().json(res)
                        }
                        $inner::Status500_ServerError(res) => {
                            actix_web::HttpResponse::InternalServerError().json(res)
                        }
                        $inner::Status503_ServiceUnavailable(res) => {
                            actix_web::HttpResponse::ServiceUnavailable().json(res)
                        }
                    }
                }
            }

            // マッピング用トレイトの定義
            trait $map_trait {
                fn map_to_response(self) -> actix_web::HttpResponse;
            }

            // MapTo$name トレイトの実装
            impl<T: Into<$entity>, E: Into<crate::models::error::ErrorContext>> $map_trait for Result<T, E> {
                fn map_to_response(self) -> actix_web::HttpResponse {
                    use std::str::FromStr;

                    // 型エイリアスの定義
                    type ErrorContext = crate::models::error::ErrorContext;
                    type ApiErrorType = crate::models::error::ApiErrorType;
                    type ProjectsList400ResponseAnyOf = smart_task_openapi_axum::models::ProjectsList400ResponseAnyOf;
                    type ProjectsList400ResponseAnyOf1 = smart_task_openapi_axum::models::ProjectsList400ResponseAnyOf1;
                    type ProjectsList400Response = smart_task_openapi_axum::models::ProjectsList400Response;
                    type ProjectsList401Response = smart_task_openapi_axum::models::ProjectsList401Response;
                    type ProjectsList403Response = smart_task_openapi_axum::models::ProjectsList403Response;
                    type ProjectsList404Response = smart_task_openapi_axum::models::ProjectsList404Response;
                    type ProjectsList429Response = smart_task_openapi_axum::models::ProjectsList429Response;
                    type ProjectsList500Response = smart_task_openapi_axum::models::ProjectsList500Response;
                    type ProjectsList503Response = smart_task_openapi_axum::models::ProjectsList503Response;

                    match self {
                        Ok(value) => {
                            let value = value.into();
                            let res = $inner::$success_status_code(value);
                            $name(res).into()
                        }
                        Err(error) => {
                            let error: ErrorContext = error.into();
                            let res = match error.error_type {
                                ApiErrorType::BadRequest => {
                                    let res = ProjectsList400ResponseAnyOf::new(
                                        1f64,
                                        ApiErrorType::BadRequest.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    let res = serde_json::to_string(&res).unwrap();
                                    let res = ProjectsList400Response::from_str(&res).unwrap();
                                    $inner::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                                }
                                ApiErrorType::Unauthorized => {
                                    let res = ProjectsList401Response::new(
                                        2f64,
                                        ApiErrorType::Unauthorized.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $inner::Status401_AccessIsUnauthorized(res)
                                }
                                ApiErrorType::Forbidden => {
                                    let res = ProjectsList403Response::new(
                                        3f64,
                                        ApiErrorType::Forbidden.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $inner::Status403_AccessIsForbidden(res)
                                }
                                ApiErrorType::NotFound => {
                                    let res = ProjectsList404Response::new(
                                        4f64,
                                        ApiErrorType::NotFound.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $inner::Status404_TheServerCannotFindTheRequestedResource(res)
                                }
                                ApiErrorType::InternalServerError => {
                                    let res = ProjectsList500Response::new(
                                        5f64,
                                        ApiErrorType::InternalServerError.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $inner::Status500_ServerError(res)
                                }
                                ApiErrorType::TooManyRequests => {
                                    let res = ProjectsList429Response::new(
                                        6f64,
                                        ApiErrorType::TooManyRequests.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $inner::Status429_ClientError(res)
                                }
                                ApiErrorType::NotDeletableResource => {
                                    let res = ProjectsList400ResponseAnyOf1::new(
                                        7f64,
                                        ApiErrorType::NotDeletableResource.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    let res = serde_json::to_string(&res).unwrap();
                                    let res = ProjectsList400Response::from_str(&res).unwrap();
                                    $inner::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                                }
                                ApiErrorType::ServiseMaintenance => {
                                    let res = ProjectsList503Response::new(
                                        8f64,
                                        ApiErrorType::ServiseMaintenance.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $inner::Status503_ServiceUnavailable(res)
                                }
                            };
                            $name(res).into()
                        }
                    }
                }
            }

        pub fn $map_function<T: Into<$entity>, E: Into<crate::models::error::ErrorContext>>(
            result: Result<T, E>,
        ) -> actix_web::HttpResponse {
            $map_trait::map_to_response(result)
        }
    };
}
