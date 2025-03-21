use crate::TestResponse;
use actix_http::body::BoxBody;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test::TestRequest;
use openapi::request::project::{ApiProjectCreateBody, ApiProjectUpdateBody};

pub struct ProjectClient {}

impl ProjectClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProjectClient {
    pub async fn create(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        request: ApiProjectCreateBody,
    ) -> TestResponse {
        let res = TestRequest::post()
            .uri("/api/v1/projects")
            .set_json(&request)
            .send_request(app)
            .await;
        TestResponse::new(res)
    }

    pub async fn get(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        id: &str,
    ) -> TestResponse {
        let res = TestRequest::get()
            .uri(&format!("/api/v1/projects/{}", id))
            .send_request(app)
            .await;
        TestResponse::new(res)
    }

    pub async fn list(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
    ) -> TestResponse {
        let res = TestRequest::get()
            .uri("/api/v1/projects")
            .send_request(app)
            .await;
        TestResponse::new(res)
    }

    pub async fn update(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        id: &str,
        request: ApiProjectUpdateBody,
    ) -> TestResponse {
        let res = TestRequest::put()
            .uri(&format!("/api/v1/projects/{}", id))
            .set_json(&request)
            .send_request(app)
            .await;
        TestResponse::new(res)
    }

    pub async fn delete(
        &self,
        app: &impl Service<Request, Response = ServiceResponse<BoxBody>, Error = impl std::fmt::Debug>,
        id: &str,
    ) -> TestResponse {
        let res = TestRequest::delete()
            .uri(&format!("/api/v1/projects/{}", id))
            .send_request(app)
            .await;
        TestResponse::new(res)
    }
}
