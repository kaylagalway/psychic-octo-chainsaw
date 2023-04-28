use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::NewUser;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = dotenv::var("DATABASE_URL").unwrap();
    
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn insert_record(new_user: NewUser) {
    let connection = &mut self::establish_connection();
    diesel::insert_into(self::schema::users::dsl::users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new post");
}
