use actix_web::{ App, HttpServer};
use users::routes::{get_user};

mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}