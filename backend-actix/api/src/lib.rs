use std::env;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, post, web};
use service::sea_orm::{Database, DatabaseConnection};
use ::entity::channel_entity;
use ::service::channel;

#[derive(Debug, Clone)]
struct ApiState {
    conn: DatabaseConnection
}

struct NotFoundDTO {
    status: String,
    path: String,
}

#[post("api/v1/channel/")]
async fn create (state: web::Data<ApiState>,
                 json_data: web::Json<channel_entity::Model>)
                 -> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let data = json_data;

    let result = ChannelService::create_channel(conn, data)
        .await
        .expect("cound not insert post");
    Ok(HttpResponse::Ok().body(result))
}

async fn not_found(data: web::Data<ApiState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().content_type("application/json").body("No Resource found at path"))
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(&db_url).await.unwrap();

    let state = ApiState {conn};

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .default_service(web::route().to(not_found))
            .configure(init)
    });

    server = server.bind(&server_url)?;
    server.run().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}")
    }
}
