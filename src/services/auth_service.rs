use crate::utils::jwt_util::generate;
use actix_web::{post, HttpResponse, Responder};

#[post("/login")]
pub async fn login() -> impl Responder {
    let username = "tefanhaetami";
    let token = generate(username);

    HttpResponse::Ok().json(token)
}
