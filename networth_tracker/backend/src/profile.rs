use crate::error::Error;
// use crate::person;
use crate::DB;
use crate::models::User;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::{get, post};
// use actix_web::{delete, get, post, put};
// use serde::Deserialize;
// use serde::Serialize;

const USER: &str = "user";

#[post("/signup")]
pub async fn signup(user: Json<User>) -> Result<Json<Vec<User>>, Error> {
    let persons: Vec<User> = DB.create(USER).content(user).await?;
    // let persons: Option<User> = DB.create((USER, "lel")).content(user).await?;
    // println!("this is lel {:?}", persons);
    // println!("this is lel {:?}", lel.unwrap());
	Ok(Json(persons))
}

#[get("/all_users")]
pub async fn get_all_users() -> Result<Json<Vec<User>>, Error> {
    println!("getting all users");
    let users = DB.select(USER).await?;
    println!("user: {:?}", users);
	Ok(Json(users))
}

#[get("/specific_user/{id}")]
pub async fn getting_specific_user(id: Path<String>) -> Result<Json<User>, Error> {
    println!("getting specific users");
    let user: Option<User> = DB.select((USER, &*id)).await?;
    match user {
        Some(user) => {Ok(Json(user))}
        None => {Err(Error::Other("User not Found".to_string()))}
    }
}
