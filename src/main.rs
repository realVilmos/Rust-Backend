use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

use crate::db::{init_pool};
use crate::auth::service::AuthService;
mod db;
mod auth;
mod schema;
mod pivot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let pool = init_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AuthService { pool: pool.clone() }))
            .configure(auth::routes::configure_v1)
            .configure(pivot::routes::configure_v1)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}