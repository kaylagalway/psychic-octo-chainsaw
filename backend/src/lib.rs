use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::sessions::dsl::*;
use dotenv::dotenv;
use models::{NewUser, User, Session};
use schema::sessions;
use diesel::result::Error;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = dotenv::var("DATABASE_URL").unwrap();
    
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn insert_user(new_user: NewUser) {
    let connection = &mut self::establish_connection();
    diesel::insert_into(self::schema::users::dsl::users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new post");
}

pub fn fetch_user(user_email: &String) -> Result<User, Error> {
    let conn = &mut self::establish_connection();
    self::schema::users::table
        .filter(self::schema::users::email.eq(user_email))
        .select(User::as_select())
        .get_result(conn)
}

pub fn insert_session(session: Session) {
    let conn = &mut self::establish_connection();
    diesel::insert_into(self::schema::sessions::dsl::sessions)
        .values(&session)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn fetch_session(other_user_id: i32) -> Result<Session, Error> {
    let conn = &mut self::establish_connection();
    self::schema::sessions::table
        .filter(self::schema::sessions::user_id.eq(other_user_id))
        .select(Session::as_select())
        .get_result(conn)
}

pub fn update_session(session: Session, new_token: &str) -> Result<usize, Error> {
    let conn = &mut self::establish_connection();
    diesel::update(sessions)
    .filter(self::schema::sessions::token.eq(session.token))
    .set(token.eq(new_token))
    .execute(conn)
}