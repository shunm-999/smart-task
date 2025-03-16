use crate::TestResponse;
use actix_http::body::BoxBody;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use openapi::request::tag::{ApiTagCreateBody, ApiTagUpdateBody};

pub struct TagClient {}

impl TagClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl TagClient {
    pub async fn get(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        tag_id: &String,
    ) -> TestResponse {
        let get_resp = test::TestRequest::get()
            .uri(&format!("/api/v1/tags/{}", tag_id))
            .send_request(app)
            .await;
        TestResponse::new(get_resp)
    }
    pub async fn list(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
    ) -> TestResponse {
        let list_resp = test::TestRequest::get()
            .uri("/api/v1/tags")
            .send_request(app)
            .await;
        TestResponse::new(list_resp)
    }
    pub async fn create(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        body: ApiTagCreateBody,
    ) -> TestResponse {
        let create_resp = test::TestRequest::post()
            .uri("/api/v1/tags")
            .set_json(&body)
            .send_request(app)
            .await;
        TestResponse::new(create_resp)
    }

    pub async fn update(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        tag_id: &String,
        body: ApiTagUpdateBody,
    ) -> TestResponse {
        let update_resp = test::TestRequest::put()
            .uri(&format!("/api/v1/tags/{}", tag_id))
            .set_json(&body)
            .send_request(app)
            .await;
        TestResponse::new(update_resp)
    }

    pub async fn delete(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        tag_id: &String,
    ) -> TestResponse {
        let delete_resp = test::TestRequest::delete()
            .uri(&format!("/api/v1/tags/{}", tag_id))
            .send_request(app)
            .await;
        TestResponse::new(delete_resp)
    }
}
