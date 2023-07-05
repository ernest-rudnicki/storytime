use actix_web::web;
use diesel::{PgConnection, RunQueryDsl};

use crate::{models::user::CreateUser, db::DbError};

pub fn create(conn: &mut PgConnection, user_data: web::Json<CreateUser>) -> Result<usize, DbError> {
    use crate::schema::users;

    let res = diesel::insert_into(users::table)
    .values(user_data.into_inner())
    .execute(conn)?;

    Ok(res)
}