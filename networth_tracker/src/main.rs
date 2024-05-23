use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use surrealdb::sql::Uuid;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
// use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
mod db;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: String,
    username: String,
    password: String,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn create_user(db: web::Data<Arc<Surreal<Db>>>, user: web::Json<User>) -> impl Responder {
    let user_id = format!("user:{}", Uuid::new_v4());
    let new_user = User {
        id: user_id.clone(),
        username: user.username.clone(),
        password: user.password.clone(),
    };

    // db.create((user_id.as_str(), new_user)).await.unwrap();

    HttpResponse::Ok().json("User created")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = db::init_db().await;
    let db = Arc::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(greet))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

