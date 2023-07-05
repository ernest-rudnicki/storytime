use std::env;

use actix_web::{ App, HttpServer, web};
use db::{create_pool};

mod users;
mod models;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(create_pool()))
            .service(users::get_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}