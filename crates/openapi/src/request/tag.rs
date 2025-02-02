use smart_task_openapi_axum::models::{
    RequestsPeriodTagPeriodCreate, TagsDeletePathParams, TagsGetPathParams, TagsListQueryParams,
    TagsUpdatePathParams,
};

pub type ApiTagsListQueryParams = TagsListQueryParams;

pub type ApiTagsGetPathParams = TagsGetPathParams;

pub type ApiTagsUpdatePathParams = TagsUpdatePathParams;

pub type ApiTagsDeletePathParams = TagsDeletePathParams;

pub type ApiTagsCreateBody = RequestsPeriodTagPeriodCreate;
