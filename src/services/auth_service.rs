use crate::db::DATABASE_NAME;
use crate::entities::credential_entity::Credential;
use crate::entities::user_entity::User;
use crate::utils::jwt_util::generate;
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection};

const COLLECTION_NAME: &str = "users";

#[post("/login")]
pub async fn login(client: web::Data<Client>, form: web::Json<Credential>) -> impl Responder {
    let username = &form.username.to_string();
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(COLLECTION_NAME);
    let user = collection.find_one(doc! { "username": &username }).await;
    match user {
        Ok(Some(_user)) => {
            let token = generate(username);
            HttpResponse::Ok().json(token)
        }
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
