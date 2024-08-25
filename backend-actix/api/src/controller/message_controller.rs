use actix_web::{delete, get, http::header::ContentType, post, put, web, Error, HttpResponse};
use entity::utils::uuid::get_zero_uuid;
use serde::Deserialize;
use service::{message::MessageService, sea_orm::prelude::Uuid};

use crate::{handle_error_internal, ApiState};

pub fn init (cfg: &mut web::ServiceConfig){
    cfg.service(create_message);
    cfg.service(get_all_messagess);
    cfg.service(get_message);
    cfg.service(update_message);
    cfg.service(delete_message);
}

#[derive(Deserialize)]
struct ChannelPathInfo{
    channel_id: Uuid
}

#[derive(Deserialize)]
struct MessagePathInfo{
    channel_id: Uuid,
    message_id: Uuid
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct MessageDTO{
    content: String,
    r#type: String
}

#[post("/api/v1/channels/{channel_id}/messages/")]
pub async fn create_message (state: web::Data<ApiState>, path: web::Path<ChannelPathInfo>)
                 -> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let channel_id = path.channel_id;


    let result = MessageService::create_message(conn,channel_id, get_zero_uuid())
        .await;
    
    println!("{:#?}",result);
    
    return if result.is_ok() {
        let json_result = serde_json::to_string(&result.unwrap());
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(json_result.expect("Failed to serialize")))
    } else {
        println!("Operation returned an Error");
        return handle_error_internal(result.unwrap_err());
    }
}

#[get("/api/v1/channels/{channel_id}/messages/")]
pub async fn get_all_messagess(state: web::Data<ApiState>, path: web::Path<ChannelPathInfo>) -> Result<HttpResponse, Error> {
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

#[get("/api/v1/channels/{channel_id}/messages/{message_id}")]
pub async fn get_message(state: web::Data<ApiState>, path: web::Path<MessagePathInfo>) -> Result<HttpResponse, Error>{
    let conn = &state.conn;
    let channel_id = path.channel_id;
    let message_id = path.message_id;

    let result = MessageService::get_message(conn,message_id,channel_id).await;
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }

    let json_result = serde_json::to_string(&result.unwrap());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_result.expect("Failed to serialize")))
}

#[put("/api/v1/channels/{channel_id}/messages/{message_id}")]
pub async fn update_message(state: web::Data<ApiState>, path: web::Path<MessagePathInfo>, data:web::Json<MessageDTO>) 
-> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let channel_id = path.channel_id;
    let message_id = path.message_id;
    
    let result = MessageService::get_message(conn,message_id,channel_id).await;
    
    if result.is_err() {
        return handle_error_internal(result.err().unwrap())
    }

    let model = result.unwrap(); 
    if model.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut unwrapped = model.unwrap();
    unwrapped.content = data.content.to_owned();
    unwrapped.r#type = data.r#type.to_owned();

    let result = MessageService::update_message(conn,channel_id,unwrapped).await;
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }
    println!("PUT Result: {:#?}", result);

    let json_result = serde_json::to_string(&result.unwrap());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_result.expect("Failed to serialize")))
}

#[delete("/api/v1/channels/{channel_id}/messages/{message_id}")]
pub async fn delete_message(state: web::Data<ApiState>, path: web::Path<MessagePathInfo>) -> Result<HttpResponse, Error>{
    let conn = &state.conn;
    let id = path.message_id;
    
    let result = MessageService::delete_message(conn, id).await;
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }

    Ok(HttpResponse::Ok().finish())
}