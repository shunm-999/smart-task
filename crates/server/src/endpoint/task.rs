use crate::dto::{MapToApiError, MapToApiModel, MapToDomain};
use crate::{map_list_to_api_response, map_to_api_response, AppData};
use actix_web::web::{Data, Query};
use actix_web::{web, HttpResponse};
use domain::repository::task::TaskRepository;
use openapi::request::task::{
    ApiTaskCreateBody, ApiTaskGetPathParams, ApiTaskUpdateBody, ApiTaskUpdatePathParams,
    ApiTasksListQueryParams,
};
use openapi::response::task::{
    map_to_create_task_response, map_to_delete_task_response, map_to_get_task_response,
    map_to_list_task_response, map_to_update_task_response,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(get_tasks)
            .service(get_task)
            .service(create_task)
            .service(update_task)
            .service(delete_task),
    );
}

#[actix_web::get("")]
async fn get_tasks(app_data: Data<AppData>, query: Query<ApiTasksListQueryParams>) -> HttpResponse {
    let tasks = app_data.repository.get_tasks().await;
    map_list_to_api_response!(tasks, map_to_list_task_response)
}

#[actix_web::get("/{id}")]
async fn get_task(app_data: Data<AppData>, path: web::Path<ApiTaskGetPathParams>) -> HttpResponse {
    let id = (&path.id).into();
    let task = app_data.repository.get_task(id).await;
    map_to_api_response!(task, map_to_get_task_response)
}

#[actix_web::post("")]
async fn create_task(
    app_data: Data<AppData>,
    web::Json(body): web::Json<ApiTaskCreateBody>,
) -> HttpResponse {
    let task_creation = body.map_to_domain();
    let task = app_data.repository.create_task(task_creation).await;
    map_to_api_response!(task, map_to_create_task_response)
}

#[actix_web::put("/{id}")]
async fn update_task(
    app_data: Data<AppData>,
    path: web::Path<ApiTaskUpdatePathParams>,
    web::Json(body): web::Json<ApiTaskUpdateBody>,
) -> HttpResponse {
    let task_update = (path.id.clone(), body).map_to_domain();
    let task = app_data.repository.update_task(task_update).await;
    map_to_api_response!(task, map_to_update_task_response)
}

#[actix_web::delete("/{id}")]
async fn delete_task(
    app_data: Data<AppData>,
    path: web::Path<ApiTaskGetPathParams>,
) -> HttpResponse {
    let id = (&path.id).into();
    let task = app_data.repository.delete_task(id).await;
    map_to_api_response!(task, map_to_delete_task_response)
}
