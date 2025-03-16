use actix_web::dev::Service;
use actix_web::{dev::ServiceFactory, dev::ServiceResponse, test, web::Data, App, Error};
use infra::SmartTaskRepositoryImpl;
use server::AppData;
use std::sync::Arc;

pub async fn setup_test_app(
) -> impl Service<actix_http::Request, Response = ServiceResponse, Error = Error> {
    let repository = SmartTaskRepositoryImpl::new("sqlite::memory:".to_string()).await;
    let app_data = Data::new(AppData {
        repository: Arc::new(repository),
    });
    let app = App::new().app_data(app_data.clone());
    test::init_service(app.configure(|cfg| {
        server::endpoint::config(cfg);
    }))
    .await
}
