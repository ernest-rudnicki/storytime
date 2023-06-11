use diesel::{Insertable};
use serde::{Deserialize};

use crate::schema::users;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}