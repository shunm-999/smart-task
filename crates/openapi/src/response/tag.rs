use crate::models::tag::ApiTag;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::tag::{
    TagsCreateResponse, TagsDeleteResponse, TagsGetResponse, TagsListResponse, TagsUpdateResponse,
};
use std::str::FromStr;

create_response!(TagsCreateResponse, ApiTag);

ok_response!(TagsGetResponse, ApiTag);
ok_response!(TagsListResponse, Vec<ApiTag>);
ok_response!(TagsUpdateResponse, ApiTag);
ok_response!(TagsDeleteResponse, ApiTag);
