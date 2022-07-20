#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

mod constants;
mod db;
mod exercise;
mod health;
mod response;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let pool = db::pool();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .service(health::health)
            .service(exercise::view::list)
        })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
