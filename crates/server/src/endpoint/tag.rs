use crate::dto::{MapToApiError, MapToApiModel};
use crate::{map_list_to_api_response, AppData};
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use domain::repository::tag::TagRepository;
use openapi::response::tag::map_to_list_tag_response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tags").service(get_tags));
}

#[actix_web::get("")]
async fn get_tags(app_data: Data<AppData>) -> HttpResponse {
    let tags = app_data.repository.get_tags().await;
    map_list_to_api_response!(tags, map_to_list_tag_response)
}
