use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};
// use surrealdb::sql::Thing;
// use surrealdb::engine::remote::http::Http;
use surrealdb::opt::auth::Root;
// use std::sync::Arc;

pub async fn init_db() -> Surreal<Db> {
    let db = Surreal::new::<Mem>(()).await.unwrap();

    db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .expect("Failed to sign in as Root");
    db.use_ns("namespace").use_db("database").await.unwrap();
    db
}

