use apolo::config::get_configuration;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, Statement};

#[tokio::main]
async fn main() {
    dbg!(get_configuration().unwrap());

    let db_conn = Database::connect("postgres://postgres:postgres@localhost:5432")
        .await
        .unwrap();

    db_conn
        .execute(Statement::from_string(
            DatabaseBackend::Postgres,
            "CREATE DATABASE banana_dev;".to_string(),
        ))
        .await
        .expect("Failed to create DB");
    
    let _db_pool = Database::connect("postgres://postgres:postgres@localhost:5432/banana_dev").await.unwrap();

    println!("Main");
}
