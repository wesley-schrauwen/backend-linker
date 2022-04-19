use actix_web::{HttpResponse, Responder, web, get};

#[get("/users")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
}