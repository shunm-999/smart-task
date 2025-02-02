use crate::dto::{MapToApiError, MapToApiModel, MapToDomain};
use crate::{map_list_to_api_response, map_to_api_response, AppData};
use actix_web::web::{Data, Query};
use actix_web::{web, HttpResponse};
use domain::repository::tag::TagRepository;
use openapi::request::tag::{
    ApiTagCreateBody, ApiTagGetPathParams, ApiTagUpdateBody, ApiTagUpdatePathParams,
    ApiTagsListQueryParams,
};
use openapi::response::tag::{
    map_to_create_tag_response, map_to_delete_tag_response, map_to_get_tag_response,
    map_to_list_tag_response, map_to_update_tag_response,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tags")
            .service(get_tags)
            .service(get_tag)
            .service(create_tag)
            .service(update_tag)
            .service(delete_tag),
    );
}

#[actix_web::get("")]
async fn get_tags(app_data: Data<AppData>, query: Query<ApiTagsListQueryParams>) -> HttpResponse {
    let tags = app_data.repository.get_tags().await;
    map_list_to_api_response!(tags, map_to_list_tag_response)
}

#[actix_web::get("/{id}")]
async fn get_tag(app_data: Data<AppData>, path: web::Path<ApiTagGetPathParams>) -> HttpResponse {
    let id = (&path.id).into();
    let tag = app_data.repository.get_tag(id).await;
    map_to_api_response!(tag, map_to_get_tag_response)
}

#[actix_web::post("")]
async fn create_tag(
    app_data: Data<AppData>,
    web::Json(body): web::Json<ApiTagCreateBody>,
) -> HttpResponse {
    let tag_creation = body.map_to_domain();
    let tag = app_data.repository.create_tag(tag_creation).await;
    map_to_api_response!(tag, map_to_create_tag_response)
}

#[actix_web::put("/{id}")]
async fn update_tag(
    app_data: Data<AppData>,
    path: web::Path<ApiTagUpdatePathParams>,
    web::Json(body): web::Json<ApiTagUpdateBody>,
) -> HttpResponse {
    let tag_update = (path.id.clone(), body).map_to_domain();
    let tag = app_data.repository.update_tag(tag_update).await;
    map_to_api_response!(tag, map_to_update_tag_response)
}

#[actix_web::delete("/{id}")]
async fn delete_tag(app_data: Data<AppData>, path: web::Path<ApiTagGetPathParams>) -> HttpResponse {
    let id = (&path.id).into();
    let tag = app_data.repository.delete_tag(id).await;
    map_to_api_response!(tag, map_to_delete_tag_response)
}
