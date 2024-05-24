mod error;
mod person;
mod schema;
mod profile;
mod models;


use actix_web::{App, HttpServer};
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	DB.connect::<Ws>("surrealdb:8000").await?;

	DB.signin(Root {
		username: "user",
		password: "password123",
	})
	.await?;

	DB.use_ns("namespace").use_db("database").await?;

    schema::create_schema(&DB).await.expect("Failed to create schema");

	HttpServer::new(|| {
		App::new()
			.service(profile::signup)
			.service(profile::get_all_users)
			.service(profile::getting_specific_user)
			// .service(profile::signin)
			.service(person::read)
			.service(person::update)
			.service(person::delete)
			.service(person::list)
			.service(person::index)
			.service(person::get_price)
	})
	.bind(("0.0.0.0", 3000))?
	.run()
	.await?;

	Ok(())
} 