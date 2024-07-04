use actix_web::{delete, get, post, put, Responder, web};

pub const ROUTE: &str = "/channels";

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE)
            .service(delete_channel)
            .service(get_channel)
            .service(get_channels)
            .service(post_channel)
            .service(put_channel));
}

#[get("/")]
async fn get_channels() -> impl Responder {

}

#[get("/{channel_id}")]
async fn get_channel(path: web::Path<String>) -> impl Responder {
    let (channel_id) = path.into_inner();
    Ok(format!("Channel {}!", channel_id))
        .expect("TODO: panic message");
}

#[post("/{channel_id}")]
async fn post_channel(path: web::Path<String>) -> impl Responder {
    let (channel_id) = path.into_inner();
    Ok(format!("Channel {}!", channel_id))
        .expect("TODO: panic message");
}

#[put("/{channel_id}")]
async fn put_channel(path: web::Path<String>) -> impl Responder {
    let (channel_id) = path.into_inner();
    Ok(format!("Channel {}!", channel_id))
        .expect("TODO: panic message");
}

#[delete("/{channel_id}")]
async fn delete_channel(path: web::Path<String>) -> impl Responder {
    let (channel_id) = path.into_inner();
    Ok(format!("Channel {}!", channel_id))
        .expect("TODO: panic message");
}