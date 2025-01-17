use dotenvy::dotenv;
use mongodb::{error::Result, Client};
use std::env;

pub const DATABASE_NAME: &str = "rust_starter";

pub async fn connect() -> Result<Client> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client = Client::with_uri_str(database_url).await?;
    Ok(client)
}
