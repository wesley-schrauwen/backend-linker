use actix_web::{HttpResponse, Responder, web, get};
use serde_json::json;
use chrono::Local;

#[get("/links")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "code": 200,
        "success": true,
        "payload": [{
            "name": "test",
            "created_at": format!("{}", Local::today()),
            "description": null,
            "link_to": null
        },{
            "name": "test_2",
            "created_at": format!("{}", Local::today()),
            "description": null,
            "link_to": null
        }]
    }))
}

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(index);
}