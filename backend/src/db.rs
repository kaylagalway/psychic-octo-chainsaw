use tokio_postgres::{NoTls, Error};
use dotenv;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let db_host = dotenv::var("host").unwrap();
    let db_user = dotenv::var("user").unwrap();
    let db_name = dotenv::var("dbname").unwrap();
    let (client, connection) = tokio_postgres::connect(&format!("host={} user={} dbname={}", db_host, db_user, db_name), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("aww shucks: {}", e);
        }
    });
    
    Ok(())
}