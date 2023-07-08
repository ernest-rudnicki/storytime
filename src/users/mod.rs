
use actix_web::{ get, Responder, Result, web, Scope, post, HttpResponse, error::ErrorInternalServerError };
use validator::Validate;

use crate::{
    models::user::{CreateUser}, 
    db::PostgresPool, 
    utils::{IdPathParams, PaginationQueryParams}, 
    users::actions::{get_action, create_action, get_multiple_action}};

mod actions;

pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(get_user)
        .service(create_user)
        .service(get_multiple_users)
}

#[get("/{id}")]
async fn get_user(pool: web::Data<PostgresPool>, params: web::Path<IdPathParams>) -> Result<impl Responder> {
    
    match params.id {
        Some(id) =>  {
            let user;

            user = web::block(move || {
                let mut conn = pool.get()?;
                
                get_action(&mut conn, &id)
            }).await?.map_err(ErrorInternalServerError)?;

            match user {
                Some(user) => Ok(HttpResponse::Ok().json(user)),
                None => Ok(HttpResponse::NotFound().body("User not found")),
            }
        },
        None => Ok(HttpResponse::BadRequest().body("User id is required")),
    }
}

#[get("")]
async fn get_multiple_users(pool: web::Data<PostgresPool>, params: web::Query<PaginationQueryParams>) -> Result<impl Responder> {
    let users;
    let is_valid_params = params.validate();

    if is_valid_params.is_err() {
        return Ok(HttpResponse::BadRequest().body(is_valid_params.unwrap_err().to_string()));
    }

    users = web::block(move || {
        let mut conn = pool.get()?;
        
        get_multiple_action(&mut conn, params.limit, params.offset)
    }).await?.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[post("")]
async fn create_user(pool: web::Data<PostgresPool>, user_data: web::Json<CreateUser>) -> Result<impl Responder> {
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
