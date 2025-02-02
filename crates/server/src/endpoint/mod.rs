mod tag;
mod task;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/v1").configure(|cfg| {
        tag::config(cfg);
    }));
}
