mod db;
mod entities;
mod services;

use actix_web::{web, App, HttpServer};
use db::connect;

use services::user_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect().await.expect("connecting to the database failed");
    user_service::generate_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(user_service::create)
            .service(user_service::find_one)
            .service(user_service::find_all)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
