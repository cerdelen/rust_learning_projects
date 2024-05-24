use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
// use surrealdb::engine
// use surrealdb::engine::remote::Db;

pub async fn create_schema(db: &Lazy<Surreal<Client>>) -> surrealdb::Result<()> {
    // user schema
    db.query("DEFINE TABLE user SCHEMAFULL;").await?;
    db.query("DEFINE FIELD username ON TABLE user TYPE string;").await?;
    db.query("DEFINE FIELD password ON TABLE user TYPE string;").await?;
    db.query("DEFINE FIELD assets ON TABLE user TYPE array<string>;").await?;
    db.query("DEFINE INDEX idIndex ON TABLE user COLUMNS id UNIQUE;").await?;

    Ok(())
}
