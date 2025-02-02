use smart_task_openapi_axum::models::{
    ProjectsDeletePathParams, ProjectsGetPathParams, ProjectsListQueryParams,
    ProjectsUpdatePathParams, RequestsPeriodTaskPeriodCreate,
};

pub type ApiProjectsListQueryParams = ProjectsListQueryParams;

pub type ApiProjectGetPathParams = ProjectsGetPathParams;

pub type ApiProjectUpdatePathParams = ProjectsUpdatePathParams;

pub type ApiProjectDeletePathParams = ProjectsDeletePathParams;

pub type ApiProjectCreateBody = RequestsPeriodTaskPeriodCreate;

pub type ApiProjectUpdateBody = RequestsPeriodTaskPeriodCreate;
