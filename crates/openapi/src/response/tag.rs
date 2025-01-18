use crate::models::error::{ApiErrorType, ErrorContext};
use crate::models::tag::ApiTag;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::tag::{
    TagsCreateResponse, TagsDeleteResponse, TagsGetResponse, TagsListResponse, TagsUpdateResponse,
};
use smart_task_openapi_axum::models::{
    ProjectsList400Response, ProjectsList400ResponseAnyOf, ProjectsList400ResponseAnyOf1,
    ProjectsList401Response, ProjectsList403Response, ProjectsList404Response,
    ProjectsList429Response, ProjectsList500Response, ProjectsList503Response,
};
use std::str::FromStr;

create_response!(TagsCreateResponse);

ok_response!(TagsGetResponse);
ok_response!(TagsListResponse);
ok_response!(TagsUpdateResponse);
ok_response!(TagsDeleteResponse);

pub trait MapToTagsCreateResponse {
    fn map_to_response(self) -> actix_web::HttpResponse;
}

impl<T: Into<ApiTag>, E: Into<ErrorContext>> MapToTagsCreateResponse for Result<T, E> {
    fn map_to_response(self) -> actix_web::HttpResponse {
        match self {
            Ok(value) => {
                let value = value.into();
                let res = TagsCreateResponse::Status201_TheRequestHasSucceededAndANewResourceHasBeenCreatedAsAResult(value);
                ApiTagsCreateResponse(res).into()
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
                        TagsCreateResponse::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                    }
                    ApiErrorType::Unauthorized => {
                        let res = ProjectsList401Response::new(
                            2f64,
                            ApiErrorType::Unauthorized.to_string(),
                            error.message,
                            error.errors,
                        );
                        TagsCreateResponse::Status401_AccessIsUnauthorized(res)
                    }
                    ApiErrorType::Forbidden => {
                        let res = ProjectsList403Response::new(
                            3f64,
                            ApiErrorType::Forbidden.to_string(),
                            error.message,
                            error.errors,
                        );
                        TagsCreateResponse::Status403_AccessIsForbidden(res)
                    }
                    ApiErrorType::NotFound => {
                        let res = ProjectsList404Response::new(
                            4f64,
                            ApiErrorType::NotFound.to_string(),
                            error.message,
                            error.errors,
                        );
                        TagsCreateResponse::Status404_TheServerCannotFindTheRequestedResource(res)
                    }
                    ApiErrorType::InternalServerError => {
                        let res = ProjectsList500Response::new(
                            5f64,
                            ApiErrorType::InternalServerError.to_string(),
                            error.message,
                            error.errors,
                        );
                        TagsCreateResponse::Status500_ServerError(res)
                    }
                    ApiErrorType::TooManyRequests => {
                        let res = ProjectsList429Response::new(
                            6f64,
                            ApiErrorType::TooManyRequests.to_string(),
                            error.message,
                            error.errors,
                        );
                        TagsCreateResponse::Status429_ClientError(res)
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
                        TagsCreateResponse::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res)
                    }
                    ApiErrorType::ServiseMaintenance => {
                        let res = ProjectsList503Response::new(
                            8f64,
                            ApiErrorType::ServiseMaintenance.to_string(),
                            error.message,
                            error.errors,
                        );
                        TagsCreateResponse::Status503_ServiceUnavailable(res)
                    }
                };
                ApiTagsCreateResponse(res).into()
            }
        }
    }
}
