use std::env;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

struct ServerState{
    db: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = Database::connect(connection_url).await?;
    let state = ServerState { db };

    let hostname = env::var("HOSTNAME").expect("HOSTNAME must be set");
    let port = env::var("PORT").expect("PORT must be set")
        .parse::<u16>().expect("PORT is not a valid number");

    HttpServer::new(|| {
        App::new()
    })
        .bind((hostname, port))?
        .run()
        .await
}
