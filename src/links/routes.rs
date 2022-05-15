use actix_web::{HttpResponse, Responder, web, get, post, put, delete, HttpMessage};
use serde_json::json;
use chrono::Local;
use uuid::Uuid;
use crate::errors::ApiError;
use crate::links::model::{CreateLinkPayload, Link, UpdateLinkPayload};

#[get("/links")]
async fn index() -> Result<HttpResponse, ApiError> {
    info!("logging back index");
    let results = Link::find_all()?;
    Ok(HttpResponse::Ok().json(results))
}

#[post("/links")]
async fn create(payload: web::Json<CreateLinkPayload>) -> Result<HttpResponse, ApiError> {
    info!("creating a new link");
    let created_link = Link::create(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(created_link))
}

#[put("/links/{link_id}")]
async fn update(link_id: web::Path<Uuid>, payload: web::Json<UpdateLinkPayload>) -> Result<HttpResponse, ApiError> {
    info!("updating link");
    let updated_link = Link::update(
        link_id.into_inner(),
        payload.into_inner()
    ).await?;
    Ok(HttpResponse::Ok().json(updated_link))
}

// #[delete("/links/:link_id")]
// async fn delete() -> impl Responder {
//     HttpResponse::Created().json(json!({
//         "code": 201,
//         "success": true
//     }))
// }

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(create);
    config.service(update);
}