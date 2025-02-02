use smart_task_openapi_axum::models::{
    RequestsPeriodTaskPeriodCreate, TasksDeletePathParams, TasksGetPathParams,
    TasksListQueryParams, TasksUpdatePathParams,
};

pub type ApiTasksListQueryParams = TasksListQueryParams;

pub type ApiTasksGetPathParams = TasksGetPathParams;

pub type ApiTasksUpdatePathParams = TasksUpdatePathParams;

pub type ApiTasksDeletePathParams = TasksDeletePathParams;

pub type ApiTasksCreateBody = RequestsPeriodTaskPeriodCreate;
