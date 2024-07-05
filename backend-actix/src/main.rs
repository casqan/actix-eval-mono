mod controller;
mod data;

use std::env;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let hostname = env::var("HOSTNAME").expect("HOSTNAME must be set");
    let port = env::var("PORT").expect("PORT must be set")
        .parse::<u16>().expect("PORT is not a valid number");

    HttpServer::new(|| {
        App::new()
            .configure(controller::channel_controller::init)
            .configure(controller::profile_controller::init)
            .configure(controller::message_controller::init)
    })
        .bind((hostname, port))?
        .run()
        .await
}
