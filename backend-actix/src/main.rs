mod services;
mod data;

use std::env;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

struct ServerState{
    db: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = Database::connect("").await?;
    let state = ServerState { db };

    let hostname = env::var("HOSTNAME").expect("HOSTNAME must be set");
    let port = env::var("PORT").expect("PORT must be set")
        .parse::<u16>().expect("PORT is not a valid number");

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(state))
            .configure(services::channel_controller::init)
            .configure(services::profile_controller::init)
            .configure(services::message_controller::init)
    })
        .bind((hostname, port))?
        .run()
        .await
}
