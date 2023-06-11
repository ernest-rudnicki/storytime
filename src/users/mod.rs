
use actix_web::{ get, Responder, Result, web, Scope, post, HttpResponse, error::ErrorInternalServerError };

use crate::{models::user::{CreateUser}, db::PostgresPool, users::actions::create};

mod actions;

pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(get_user)
        .service(create_user)
}

#[get("")]
async fn get_user() -> Result<impl Responder> {
    Ok(web::Json("Hello world"))
}

#[post("")]
async fn create_user(pool: web::Data<PostgresPool>, user_data: web::Json<CreateUser>) -> Result<impl Responder> {

    web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        create(&mut conn, user_data)
    }).await?
    .map_err(ErrorInternalServerError)?;
 

    Ok(HttpResponse::Created())

}