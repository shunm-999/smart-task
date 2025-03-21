pub(crate) mod client;
pub(crate) mod endpoint;

use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use serde::de::DeserializeOwned;

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

    fn status(&self) -> u16 {
        self.response.status().as_u16()
    }

    async fn to_body<T: DeserializeOwned>(self) -> T {
        test::read_body_json(self.response).await
    }
}
