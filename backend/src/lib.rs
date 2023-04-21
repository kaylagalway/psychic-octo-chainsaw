use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = dotenv::var("DATABASE_URL").unwrap();
    
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
