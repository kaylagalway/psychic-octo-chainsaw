use diesel::{prelude::*};

#[derive(Queryable)]
pub struct User {
    pub displayname: String,
    pub email: String,
    pub id: i32,
    pub passhash: String
}

#[derive(Queryable)]

pub struct Session {
    pub token: String,
    pub exp_date: i64,
    pub user_id: i32
}