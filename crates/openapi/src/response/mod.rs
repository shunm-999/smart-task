pub mod project;
pub mod tag;
pub mod task;

#[macro_export]
macro_rules! create_response {
    ($name:ident, $entity:ty) => {
        paste::paste! {
            // 動的に構造体を定義
            pub struct [<Api $name>]($name);

            // From トレイトの実装
            impl From<[<Api $name>]> for actix_web::HttpResponse {
                fn from(value: [<Api $name>]) -> Self {
                    let value = value.0;

                    match value {
                        $name::Status201_TheRequestHasSucceededAndANewResourceHasBeenCreatedAsAResult(res) => {
                            actix_web::HttpResponse::Created().json(res)
                        }
                        $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            actix_web::HttpResponse::BadRequest().json(res)
                        }
                        $name::Status401_AccessIsUnauthorized(res) => {
                            actix_web::HttpResponse::Unauthorized().json(res)
                        }
                        $name::Status403_AccessIsForbidden(res) => {
                            actix_web::HttpResponse::Forbidden().json(res)
                        }
                        $name::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            actix_web::HttpResponse::NotFound().json(res)
                        }
                        $name::Status429_ClientError(res) => {
                            actix_web::HttpResponse::TooManyRequests().json(res)
                        }
                        $name::Status500_ServerError(res) => {
                            actix_web::HttpResponse::InternalServerError().json(res)
                        }
                        $name::Status503_ServiceUnavailable(res) => {
                            actix_web::HttpResponse::ServiceUnavailable().json(res)
                        }
                    }
                }
            }

            // マッピング用トレイトの定義
            pub trait [<MapTo $name>] {
                fn map_to_response(self) -> actix_web::HttpResponse;
            }

            // MapTo$name トレイトの実装
            impl<T: Into<$entity>, E: Into<crate::models::error::ErrorContext>> [<MapTo $name>] for Result<T, E> {
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
                            let res = $name::Status201_TheRequestHasSucceededAndANewResourceHasBeenCreatedAsAResult(value);
                            [<Api $name>](res).into()
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
                                    $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                                }
                                ApiErrorType::Unauthorized => {
                                    let res = ProjectsList401Response::new(
                                        2f64,
                                        ApiErrorType::Unauthorized.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status401_AccessIsUnauthorized(res)
                                }
                                ApiErrorType::Forbidden => {
                                    let res = ProjectsList403Response::new(
                                        3f64,
                                        ApiErrorType::Forbidden.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status403_AccessIsForbidden(res)
                                }
                                ApiErrorType::NotFound => {
                                    let res = ProjectsList404Response::new(
                                        4f64,
                                        ApiErrorType::NotFound.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status404_TheServerCannotFindTheRequestedResource(res)
                                }
                                ApiErrorType::InternalServerError => {
                                    let res = ProjectsList500Response::new(
                                        5f64,
                                        ApiErrorType::InternalServerError.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status500_ServerError(res)
                                }
                                ApiErrorType::TooManyRequests => {
                                    let res = ProjectsList429Response::new(
                                        6f64,
                                        ApiErrorType::TooManyRequests.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status429_ClientError(res)
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
                                    $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                                }
                                ApiErrorType::ServiseMaintenance => {
                                    let res = ProjectsList503Response::new(
                                        8f64,
                                        ApiErrorType::ServiseMaintenance.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status503_ServiceUnavailable(res)
                                }
                            };
                            [<Api $name>](res).into()
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ok_response {
    ($name:ident, $entity:ty) => {
        paste::paste! {
            // 動的に構造体を定義
            pub struct [<Api $name>]($name);

            // From トレイトの実装
            impl From<[<Api $name>]> for actix_web::HttpResponse {
                fn from(value: [<Api $name>]) -> Self {
                    let value = value.0;

                    match value {
                        $name::Status200_TheRequestHasSucceeded(res) => {
                            actix_web::HttpResponse::Ok().json(res)
                        }
                        $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            actix_web::HttpResponse::BadRequest().json(res)
                        }
                        $name::Status401_AccessIsUnauthorized(res) => {
                            actix_web::HttpResponse::Unauthorized().json(res)
                        }
                        $name::Status403_AccessIsForbidden(res) => {
                            actix_web::HttpResponse::Forbidden().json(res)
                        }
                        $name::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            actix_web::HttpResponse::NotFound().json(res)
                        }
                        $name::Status429_ClientError(res) => {
                            actix_web::HttpResponse::TooManyRequests().json(res)
                        }
                        $name::Status500_ServerError(res) => {
                            actix_web::HttpResponse::InternalServerError().json(res)
                        }
                        $name::Status503_ServiceUnavailable(res) => {
                            actix_web::HttpResponse::ServiceUnavailable().json(res)
                        }
                    }
                }
            }

            // マッピング用トレイトの定義
            pub trait [<MapTo $name>] {
                fn map_to_response(self) -> actix_web::HttpResponse;
            }

            // MapTo$name トレイトの実装
            impl<T: Into<$entity>, E: Into<crate::models::error::ErrorContext>> [<MapTo $name>] for Result<T, E> {
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
                            let res = $name::Status200_TheRequestHasSucceeded(value);
                            [<Api $name>](res).into()
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
                                    $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                                }
                                ApiErrorType::Unauthorized => {
                                    let res = ProjectsList401Response::new(
                                        2f64,
                                        ApiErrorType::Unauthorized.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status401_AccessIsUnauthorized(res)
                                }
                                ApiErrorType::Forbidden => {
                                    let res = ProjectsList403Response::new(
                                        3f64,
                                        ApiErrorType::Forbidden.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status403_AccessIsForbidden(res)
                                }
                                ApiErrorType::NotFound => {
                                    let res = ProjectsList404Response::new(
                                        4f64,
                                        ApiErrorType::NotFound.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status404_TheServerCannotFindTheRequestedResource(res)
                                }
                                ApiErrorType::InternalServerError => {
                                    let res = ProjectsList500Response::new(
                                        5f64,
                                        ApiErrorType::InternalServerError.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status500_ServerError(res)
                                }
                                ApiErrorType::TooManyRequests => {
                                    let res = ProjectsList429Response::new(
                                        6f64,
                                        ApiErrorType::TooManyRequests.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status429_ClientError(res)
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
                                    $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                                }
                                ApiErrorType::ServiseMaintenance => {
                                    let res = ProjectsList503Response::new(
                                        8f64,
                                        ApiErrorType::ServiseMaintenance.to_string(),
                                        error.message,
                                        error.errors,
                                    );
                                    $name::Status503_ServiceUnavailable(res)
                                }
                            };
                            [<Api $name>](res).into()
                        }
                    }
                }
            }
        }
    };
}
