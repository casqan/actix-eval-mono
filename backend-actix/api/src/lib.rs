use std::env;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web};

struct ApiState {}
struct NotFoundDTO{
    status : String,
    path: String
}

async fn not_found(data: web::Data<ApiState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let body = NotFoundDTO {
        status: "No requested resource found at path.".to_owned(),
        path : request.path().to_owned()
    };

    Ok(HttpResponse::NotFound().content_type("application/json").body(body))
}

fn init(cfg: &mut web::ServiceConfig){

}

#[actix_web::main]
pub async fn start() -> std::io::Result<()>{

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let server = HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(not_found))
            .configure(init)
    });

    server.bind(&server_url)?;
    server.run().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err(){
        println!("Error: {err}")
    }
}
