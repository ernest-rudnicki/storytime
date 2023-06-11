use actix_web::web;
use diesel::{PgConnection, RunQueryDsl};

use crate::models::user::CreateUser;

pub fn create(conn: &mut PgConnection, user_data: web::Json<CreateUser>) -> Result<usize, diesel::result::Error> {
    use crate::schema::users;

    diesel::insert_into(users::table)
    .values(user_data.into_inner())
    .execute(conn)
}