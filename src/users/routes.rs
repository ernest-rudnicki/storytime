
use actix_web::{ get, Responder, Result, web };
use super::models::User;

#[get("/users")]
async fn get_user() -> Result<impl Responder> {
    let user = User {
        id: 1,
        name: "John".to_string(),
        email: "asd@gmail.com".to_string(),
    };

    Ok(web::Json(user))
}