use actix_web::{get, http::header::ContentType, post, web, Error, HttpResponse};
use ::entity::channel_entity;
use serde::Deserialize;
use ::service::channel::ChannelService;
use service::sea_orm::{prelude::Uuid, Statement};

use crate::{handle_error_internal, ApiState};

pub fn init (cfg: &mut web::ServiceConfig){
    cfg.service(create);
    cfg.service(get_all);
}

#[post("api/v1/channels/")]
pub async fn create (state: web::Data<ApiState>,
                 json_data: web::Json<channel_entity::Model>)
                 -> Result<HttpResponse, Error> {
    let conn = &state.conn;
    let data = json_data.into_inner();

    let result = ChannelService::create_channel(conn)
        .await;
    
    println!("{:#?}",result);
    
    return if result.is_ok() {
        let json_result = serde_json::to_string(&result.unwrap());
        Ok(HttpResponse::Ok()
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

    return if result.is_ok() {
        let json_result = serde_json::to_string(&result.unwrap());
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json_result.expect("Failed to serialize")))
    } else {
        println!("Operation returned an Error");
        return handle_error_internal(result.unwrap_err());
    }
}

#[derive(Deserialize)]
struct GetPathInfo{
    channel_id: Uuid
}

#[get("/api/v1/channels/{id}")]
pub async fn get(state: web::Data<ApiState>, path: web::Path<GetPathInfo>) -> Result<HttpResponse, Error>{
    let conn = &state.conn;
    let id = path.channel_id;

    let result = ChannelService::get_channel(conn,id).await;

    return if result.is_ok() {
        let json_result = serde_json::to_string(&result.unwrap());
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json_result.expect("Failed to serialize")))
    } else {
        println!("Operation returned an Error");
        return handle_error_internal(result.unwrap_err());
    }
}