use actix_web::{delete, get, post, put, Responder, web};

pub const ROUTE: &str = "/users";

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE)
            .service(delete_profile)
            .service(get_profile)
            .service(get_profiles)
            .service(post_profile)
            .service(put_profile));
}

#[get("/{profile_id}")]
async fn get_profile(path: web::Path<String>) -> impl Responder {
    let (profile_id) = path.into_inner();
}

#[get("/")]
async fn get_profiles(path: web::Path<String>) -> impl Responder {
    let (profile_id) = path.into_inner();
}

#[post("/{profile_id}")]
async fn post_profile(path: web::Path<String>) -> impl Responder {
    let (profile_id) = path.into_inner();
}

#[put("/{profile_id}")]
async fn put_profile(path: web::Path<String>) -> impl Responder {
    let (profile_id) = path.into_inner();
}

#[delete("/{profile_id}")]
async fn delete_profile(path: web::Path<String>) -> impl Responder {
    let (profile_id) = path.into_inner();
}