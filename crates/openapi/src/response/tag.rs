use crate::{create_response, ok_response};
use actix_web::HttpResponse;
use smart_task_openapi_axum::apis::tag::{
    TagsCreateResponse, TagsDeleteResponse, TagsGetResponse, TagsListResponse, TagsUpdateResponse,
};

create_response!(TagsCreateResponse);

ok_response!(TagsGetResponse);
ok_response!(TagsListResponse);
ok_response!(TagsUpdateResponse);
ok_response!(TagsDeleteResponse);
