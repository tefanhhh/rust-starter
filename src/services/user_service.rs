use crate::db::DATABASE_NAME;
use crate::entities::user_entity::User;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

const COLLECTION_NAME: &str = "users";

#[post("/users")]
pub async fn create(client: web::Data<Client>, form: web::Json<User>) -> impl Responder {
    let collection = client.database(DATABASE_NAME).collection(COLLECTION_NAME);
    let user = collection.insert_one(form.into_inner()).await;
    match user {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn find_all(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(COLLECTION_NAME);
    let cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    };
    let mut users = Vec::new();
    let mut cursor = cursor;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => users.push(user),
            Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
    }

    HttpResponse::Ok().json(users)
}

#[get("/users/{username}")]
pub async fn find_one(client: web::Data<Client>, username: web::Path<String>) -> impl Responder {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(COLLECTION_NAME);
    let user = collection.find_one(doc! { "username": &username }).await;
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn generate_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DATABASE_NAME)
        .collection::<User>(COLLECTION_NAME)
        .create_index(model)
        .await
        .expect("creating an index should succeed");
}
