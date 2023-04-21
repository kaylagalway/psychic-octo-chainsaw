use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub displayname: String,
    pub email: String,
    pub id: i32,
    pub passhash: String
}