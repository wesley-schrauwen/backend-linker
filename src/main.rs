#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger;

mod users;
mod links;
mod errors;
mod db;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init();

    let port = 4000;
    let address = "0.0.0.0";

    info!("Env validated. Building server");

    let server = HttpServer::new(|| App::new()
        // .configure(users::init_routes)
        .configure(links::routes)
    ).bind((address, port))?;

    info!("Server successfully built. Launching on {}:{}", address, port);

    return server.run().await;
}
