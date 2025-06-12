use actix_web::web;

use crate::auth::handler::{register, login, get_authenticated_endpoint};

pub fn configure_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-v1")
            .service(register)
            .service(login)
            .service(get_authenticated_endpoint)
    );
}