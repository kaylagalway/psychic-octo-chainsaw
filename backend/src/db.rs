use tokio_postgres::{NoTls, Error};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=deff dbname=party", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("aww shucks: {}", e);
        }
    });
    
    Ok(())
}