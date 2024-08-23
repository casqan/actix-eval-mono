use actix_web::{delete, get, http::header::ContentType, post, put, web, Error, HttpResponse};
use serde::Deserialize;
use ::service::channel::ChannelService;
use service::sea_orm::prelude::Uuid;

use crate::{handle_error_internal, ApiState};

pub fn init (cfg: &mut web::ServiceConfig){
    cfg.service(create);
    cfg.service(get_all);
    cfg.service(get);
    cfg.service(put);
    cfg.service(delete);
}

#[derive(Deserialize)]
struct ChannelPathInfo{
    channel_id: Uuid
}


#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct ChannelDTO{
    name: String,
    description: String,
    is_public: bool
}

#[post("api/v1/channels/")]
pub async fn create (state: web::Data<ApiState>)
                 -> Result<HttpResponse, Error> {
    let conn = &state.conn;

    let result = ChannelService::create_channel(conn)
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

#[get("/api/v1/channels/")]
pub async fn get_all(state: web::Data<ApiState>) -> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let result = ChannelService::get_all_channels(conn).await;
    
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }

    let json_result = serde_json::to_string(&result.unwrap());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_result.expect("Failed to serialize")))
}

#[get("/api/v1/channels/{channel_id}")]
pub async fn get(state: web::Data<ApiState>, path: web::Path<ChannelPathInfo>) -> Result<HttpResponse, Error>{
    let conn = &state.conn;
    let id = path.channel_id;

    let result = ChannelService::get_channel(conn,id).await;
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }

    let json_result = serde_json::to_string(&result.unwrap());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_result.expect("Failed to serialize")))
}

#[put("/api/v1/channels/{channel_id}")]
pub async fn put(state: web::Data<ApiState>, path: web::Path<ChannelPathInfo>, data:web::Json<ChannelDTO>) 
-> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let id = path.channel_id;
    
    let result = ChannelService::get_channel(conn,id).await;
    
    if result.is_err() {
        return handle_error_internal(result.err().unwrap())
    }

    let model = result.unwrap(); 
    if model.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let mut unwrapped = model.unwrap();
    unwrapped.description = data.description.to_owned();
    unwrapped.name = data.name.to_owned();
    unwrapped.is_public = data.is_public;

    let result = ChannelService::update_channel(conn,unwrapped).await;
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }
    println!("PUT Result: {:#?}", result);

    let json_result = serde_json::to_string(&result.unwrap());
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_result.expect("Failed to serialize")))
}

#[delete("/api/v1/channels/{channel_id}")]
pub async fn delete(state: web::Data<ApiState>, path: web::Path<ChannelPathInfo>) -> Result<HttpResponse, Error>{
    let conn = &state.conn;
    let id = path.channel_id;
    
    let result = ChannelService::delete_channel(conn, id).await;
    if result.is_err() {
        return handle_error_internal(result.unwrap_err());
    }

    Ok(HttpResponse::Ok().finish())
}