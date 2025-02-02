use crate::dto::{MapToApiError, MapToApiModel, MapToDomain};
use crate::{map_list_to_api_response, map_to_api_response, AppData};
use actix_web::web::{Data, Query};
use actix_web::{web, HttpResponse};
use domain::repository::project::ProjectRepository;
use openapi::request::project::{
    ApiProjectCreateBody, ApiProjectGetPathParams, ApiProjectUpdateBody,
    ApiProjectUpdatePathParams, ApiProjectsListQueryParams,
};
use openapi::response::project::{
    map_to_create_project_response, map_to_delete_project_response, map_to_get_project_response,
    map_to_list_project_response, map_to_update_project_response,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .service(get_projects)
            .service(get_project)
            .service(create_project)
            .service(update_project)
            .service(delete_project),
    );
}

#[actix_web::get("")]
async fn get_projects(
    app_data: Data<AppData>,
    query: Query<ApiProjectsListQueryParams>,
) -> HttpResponse {
    let projects = app_data.repository.get_projects().await;
    map_list_to_api_response!(projects, map_to_list_project_response)
}

#[actix_web::get("/{id}")]
async fn get_project(
    app_data: Data<AppData>,
    path: web::Path<ApiProjectGetPathParams>,
) -> HttpResponse {
    let id = (&path.id).into();
    let project = app_data.repository.get_project(id).await;
    map_to_api_response!(project, map_to_get_project_response)
}

#[actix_web::post("")]
async fn create_project(
    app_data: Data<AppData>,
    web::Json(body): web::Json<ApiProjectCreateBody>,
) -> HttpResponse {
    let project_creation = body.map_to_domain();
    let project = app_data.repository.create_project(project_creation).await;
    map_to_api_response!(project, map_to_create_project_response)
}

#[actix_web::put("/{id}")]
async fn update_project(
    app_data: Data<AppData>,
    path: web::Path<ApiProjectUpdatePathParams>,
    web::Json(body): web::Json<ApiProjectUpdateBody>,
) -> HttpResponse {
    let project_update = (path.id.clone(), body).map_to_domain();
    let project = app_data.repository.update_project(project_update).await;
    map_to_api_response!(project, map_to_update_project_response)
}

#[actix_web::delete("/{id}")]
async fn delete_project(
    app_data: Data<AppData>,
    path: web::Path<ApiProjectGetPathParams>,
) -> HttpResponse {
    let id = (&path.id).into();
    let project = app_data.repository.delete_project(id).await;
    map_to_api_response!(project, map_to_delete_project_response)
}
