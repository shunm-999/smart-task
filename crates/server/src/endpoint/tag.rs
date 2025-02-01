use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tags").service(get_channels));
}

#[actix_web::get("")]
async fn get_channels() -> impl actix_web::Responder {
    "Hello, world!"
}
