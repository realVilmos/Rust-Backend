use actix_web::web;

use crate::pivot::handler::{};

pub fn configure_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-v1")
            .service(register)
            .service(login)
            .service(get_authenticated_endpoint)
    );
}