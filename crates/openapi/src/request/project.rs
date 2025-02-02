use smart_task_openapi_axum::models::{
    ProjectsDeletePathParams, ProjectsGetPathParams, ProjectsListQueryParams,
    ProjectsUpdatePathParams, RequestsPeriodTaskPeriodCreate,
};

pub type ApiProjectsListQueryParams = ProjectsListQueryParams;

pub type ApiProjectsGetPathParams = ProjectsGetPathParams;

pub type ApiProjectsUpdatePathParams = ProjectsUpdatePathParams;

pub type ApiProjectsDeletePathParams = ProjectsDeletePathParams;

pub type ApiProjectsCreateBody = RequestsPeriodTaskPeriodCreate;
