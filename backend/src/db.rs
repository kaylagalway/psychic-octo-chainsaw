use tokio_postgres::{NoTls, Error, Client};
use dotenv::dotenv;

#[tokio::main]
pub async fn main() -> Result<Client, Error> {
    dotenv().ok();

    let db_host = dotenv::var("host").unwrap();
    let db_user = dotenv::var("user").unwrap();
    let db_name = dotenv::var("dbname").unwrap();
    let db_result = tokio_postgres::connect(
        &format!(
            "host={} user={} dbname={}", db_host, db_user, db_name
        ), 
        NoTls
    ).await;

    match db_result {
        Ok((client, _connection)) => {
            return Ok(client);
        }
        Err(error) => {
            return Err(error);
        }
    };
}