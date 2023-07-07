
use actix_web::{ get, Responder, Result, web, Scope, post, HttpResponse, Error, error::ErrorInternalServerError };
use validator::Validate;

use crate::{models::user::{CreateUser}, db::PostgresPool, utils::IdParams, users::actions::{get_action, create_action}};

mod actions;

pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(get_user)
        .service(create_user)
}

#[get("/{id}")]
async fn get_user(pool: web::Data<PostgresPool>, params: web::Path<IdParams>) -> Result<impl Responder> {
    
    match params.id {
        Some(id) =>  {
            let user;

            user = web::block(move || {
                let mut conn = pool.get()?;
                
                get_action(&mut conn, &id)
            }).await?.map_err(ErrorInternalServerError)?;
        
            Ok(HttpResponse::Ok().json(user))
        },
        None => return Ok(HttpResponse::BadRequest().body("User id is required")),
    }
}

#[post("")]
async fn create_user(pool: web::Data<PostgresPool>, user_data: web::Json<CreateUser>) -> Result<HttpResponse, Error> {
    let is_valid = user_data.validate();

    if is_valid.is_err() {
        return Ok(HttpResponse::BadRequest().body(is_valid.unwrap_err().to_string()));
    }

    web::block(move || {
        let mut conn = pool.get()?;

        create_action(&mut conn, user_data)
    }).await?
    .map_err(ErrorInternalServerError)?;
 

    Ok(HttpResponse::Created().body("User created"))
}
