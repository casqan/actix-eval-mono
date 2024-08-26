use actix_web::{Error, HttpResponse, post, web};
use actix_web::http::header::ContentType;
use service::auth;
use service::message::MessageService;
use crate::{ApiState, handle_error_internal};

#[post("/api/v1/signup/")]
pub async fn signup(state: web::Data<ApiState>) -> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let channel_id = path.channel_id;
    let result = MessageService::get_all_messagess(conn,channel_id).await;

    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }

    let json_result = serde_json::to_string(&result.unwrap());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_result.expect("Failed to serialize")))
}

#[post("/api/v1/signin/")]
pub async fn signin(state: web::Data<ApiState>) -> Result<HttpResponse, Error> {

}