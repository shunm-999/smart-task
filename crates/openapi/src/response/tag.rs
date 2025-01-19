use crate::models::tag::ApiTag;
use crate::{create_response, ok_response};
use smart_task_openapi_axum::apis::tag::{
    TagsCreateResponse, TagsDeleteResponse, TagsGetResponse, TagsListResponse, TagsUpdateResponse,
};
use std::str::FromStr;

create_response!(
    ApiTagsCreateResponse,
    TagsCreateResponse,
    MapToTagsCreateResponse,
    ApiTag,
    map_to_create_tag_response
);

ok_response!(
    ApiTagsGetResponse,
    TagsGetResponse,
    MapToTagsGetResponse,
    ApiTag,
    map_to_get_tag_response
);
ok_response!(
    ApiTagsListResponse,
    TagsListResponse,
    MapToTagsListResponse,
    Vec<ApiTag>,
    map_to_list_tag_response
);
ok_response!(
    ApiTagsUpdateResponse,
    TagsUpdateResponse,
    MapToTagsUpdateResponse,
    ApiTag,
    map_to_update_tag_response
);
ok_response!(
    ApiTagsDeleteResponse,
    TagsDeleteResponse,
    MapToTagsDeleteResponse,
    ApiTag,
    map_to_delete_tag_response
);
