use crate::environment::{load_env, Environment};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use infra::SmartTaskRepositoryImpl;
use server::SmartTaskServer;
use std::sync::Arc;

mod environment;

#[actix_web::main]
async fn main() {
    let env = load_env();
    start_server(env).await;
}
async fn start_server(env: Environment) {
    let repository = SmartTaskRepositoryImpl::new(env.database_url()).await;
    let server = SmartTaskServer {
        repository: Arc::new(repository),
    };

    let socket_addr = env.server_url();

    HttpServer::new(move || {
        let app = App::new().app_data(Data::new(server.clone()));
        app.configure(|cfg| {
            server::endpoint::config(cfg);
        })
    })
    .bind(socket_addr)
    .expect("Failed to bind to socket address")
    .run()
    .await
    .expect("Failed to run server");
}
