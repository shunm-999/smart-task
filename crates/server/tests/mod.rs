pub(crate) mod client;
pub(crate) mod endpoint;

use actix_http::body::BoxBody;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::web::Data;
use actix_web::{test, App};
use infra::SmartTaskRepositoryImpl;
use serde::de::DeserializeOwned;
use server::AppData;
use std::sync::Arc;

// pub(crate) async fn config<S, B>() -> S
// where
//     S: Service<Request, Response = ServiceResponse<BoxBody>, Error = B>,
//     B: std::fmt::Debug,
// {
//     let repository = SmartTaskRepositoryImpl::new("sqlite::memory:".to_string()).await;
//     let app_data = Data::new(AppData {
//         repository: Arc::new(repository),
//     });
//     let app = App::new().app_data(app_data.clone());
//
//     test::init_service(app.configure(|cfg| {
//         server::endpoint::config(cfg);
//     }))
//     .await
// }

pub struct TestResponse {
    pub response: ServiceResponse,
}

impl TestResponse {
    fn new(response: ServiceResponse) -> Self {
        Self { response }
    }
    fn is_success(&self) -> bool {
        self.response.status().is_success()
    }

    async fn to_body<T: DeserializeOwned>(self) -> T {
        test::read_body_json(self.response).await
    }
}
