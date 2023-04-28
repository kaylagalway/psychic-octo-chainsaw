use diesel::{prelude::*};
use rocket::serde::{Deserialize, Serialize};
use super::schema::*;

#[derive(Queryable)]
// #[diesel(table_name = users)]
pub struct User {
    pub display_name: String,
    pub email: String,
    pub id: i32,
    pub passhash: String
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub passhash: String,
    pub display_name: String
}

#[derive(Queryable)]
pub struct Session {
    pub token: String,
    pub exp_date: i64,
    pub user_id: i32
}