use actix_web::{delete, get, post, put, Responder, web};

pub const ROUTE: &str = "/channels";

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE)
            .service(delete_message)
            .service(get_message)
            .service(get_messages)
            .service(post_message)
            .service(put_message));
}

#[get("/{channel_id}/messages")]
async fn get_messages() -> impl Responder {

}

#[get("/{channel_id}/messages/{message_id}")]
async fn get_message(path: web::Path<(String, String)>) -> impl Responder {
    let (channel_id, message_id) = path.into_inner();
    Ok(format!("Channel {}", channel_id))
        .expect("TODO: panic message");
}

#[post("/{channel_id}/messages/{message_id}")]
async fn post_message(path: web::Path<(String, String)>) -> impl Responder {
    let (channel_id, message_id) = path.into_inner();
    Ok(format!("Channel {}", channel_id))
        .expect("TODO: panic message");
}

#[put("/{channel_id}/messages/{message_id}")]
async fn put_message(path: web::Path<(String, String)>) -> impl Responder {
    let (channel_id, message_id) = path.into_inner();
    Ok(format!("Channel {}", channel_id))
        .expect("TODO: panic message");
}

#[delete("/{channel_id}/messages/{message_id}")]
async fn delete_message(path: web::Path<(String, String)>) -> impl Responder {
    let (channel_id, message_id) = path.into_inner();
    Ok(format!("Channel {}", channel_id))
        .expect("TODO: panic message");
}