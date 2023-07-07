use actix_web::web;
use diesel::{PgConnection, RunQueryDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{models::user::{CreateUser, User}, db::DbError};

pub fn create_action(conn: &mut PgConnection, user_data: web::Json<CreateUser>) -> Result<usize, DbError> {
    use crate::schema::users;

    let res = diesel::insert_into(users::table)
    .values(user_data.into_inner())
    .execute(conn)?;

    Ok(res)
}

pub fn get_action(conn: &mut PgConnection, user_id: &Uuid) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let res = users.filter(id.eq(user_id)).select(User::as_select()).first(conn)?;

    Ok(res)
}