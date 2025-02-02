use smart_task_openapi_axum::models::{
    RequestsPeriodTagPeriodCreate, RequestsPeriodTagPeriodUpdate, TagsDeletePathParams,
    TagsGetPathParams, TagsListQueryParams, TagsUpdatePathParams,
};

pub type ApiTagsListQueryParams = TagsListQueryParams;

pub type ApiTagGetPathParams = TagsGetPathParams;

pub type ApiTagUpdatePathParams = TagsUpdatePathParams;

pub type ApiTagDeletePathParams = TagsDeletePathParams;

pub type ApiTagCreateBody = RequestsPeriodTagPeriodCreate;

pub type ApiTagUpdateBody = RequestsPeriodTagPeriodUpdate;
