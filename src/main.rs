mod db;
mod entities;
mod middlewares;
mod services;
mod utils;

use actix_web::{web, App, HttpServer};
use db::connect;

use middlewares::auth_middleware;
use services::auth_service;
use services::user_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect().await.expect("connecting to the database failed");
    user_service::generate_index(&client).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            // auth
            .service(auth_service::login)
            // Protected routes
            .service(
                web::scope("")
                    // middleware
                    .wrap(auth_middleware::AuthMiddleware)
                    // user
                    .service(user_service::create)
                    .service(user_service::find_one)
                    .service(user_service::find_all),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
