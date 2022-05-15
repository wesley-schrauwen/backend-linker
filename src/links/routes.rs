use actix_web::{HttpResponse, Responder, web, get, post, put, delete};
use serde_json::json;
use chrono::Local;
use crate::errors::ApiError;
use crate::links::model::{CreateLinkPayload, Link};

#[get("/links")]
async fn index() -> Result<HttpResponse, ApiError> {
    let results = Link::find_all()?;
    info!("logging back index");
    Ok(HttpResponse::Ok().json(results))
}

#[post("/links")]
async fn create(link_payload: web::Json<CreateLinkPayload>) -> Result<HttpResponse, ApiError> {
    let created_link = Link::create(link_payload.into_inner())?;
    Ok(HttpResponse::Created().json(created_link))
}

// #[put("/links/:link_id")]
// async fn update() -> impl Responder {
//     HttpResponse::Created().json(json!({
//         "code": 201,
//         "success": true
//     }))
// }
//
// #[delete("/links/:link_id")]
// async fn delete() -> impl Responder {
//     HttpResponse::Created().json(json!({
//         "code": 201,
//         "success": true
//     }))
// }

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(index);
}