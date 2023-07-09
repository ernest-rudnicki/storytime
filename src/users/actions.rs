use actix_web::web;
use diesel::{PgConnection, RunQueryDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use uuid::Uuid;
use diesel::OptionalExtension;

use crate::{models::user::{CreateUser, User}, db::DbError};

pub fn create_action(conn: &mut PgConnection, user_data: web::Json<CreateUser>) -> Result<usize, DbError> {
    use crate::schema::users;

    let res = diesel::insert_into(users::table)
    .values(user_data.into_inner())
    .execute(conn)?;

    Ok(res)
}

pub fn get_action(conn: &mut PgConnection, user_id: &Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let res = users.find(user_id).select(User::as_select()).first(conn).optional()?;

    Ok(res)
}

pub fn get_multiple_action(conn: &mut PgConnection, limit: i64, offset: i64) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;

    let res = users.select(User::as_select()).order(created_at.desc()).limit(limit).offset(offset).load(conn)?;

    Ok(res)

}