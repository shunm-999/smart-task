use smart_task_openapi_axum::models::{
    RequestsPeriodTaskPeriodCreate, RequestsPeriodTaskPeriodUpdate, TasksDeletePathParams,
    TasksGetPathParams, TasksListQueryParams, TasksUpdatePathParams,
};

pub type ApiTasksListQueryParams = TasksListQueryParams;

pub type ApiTaskGetPathParams = TasksGetPathParams;

pub type ApiTaskUpdatePathParams = TasksUpdatePathParams;

pub type ApiTaskDeletePathParams = TasksDeletePathParams;

pub type ApiTaskCreateBody = RequestsPeriodTaskPeriodCreate;

pub type ApiTaskUpdateBody = RequestsPeriodTaskPeriodUpdate;
