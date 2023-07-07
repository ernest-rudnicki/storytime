use chrono::NaiveDateTime;
use diesel::{Insertable, Selectable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


use crate::{schema::users};

#[derive(Deserialize, Insertable, Validate)]
#[diesel(table_name = users)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 24, message = "Name must be between 3 and 24 characters"))]
    pub name: String,
    #[validate(email(message = "Email is not valid"), length(max = 100, message = "Email must be less than 100 characters"))]
    pub email: String,
    #[validate(length(min = 8, max = 24, message = "Name must be between 8 and 24 characters"))]
    pub password: String,
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}
