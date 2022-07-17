#[macro_use]
extern crate actix_web;

use std::{io};

use actix_web::{middleware, App, HttpServer};

mod constants;
mod health;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // Set up DB pool to be used with web::Data<Pool> extractor
            // .data(pool.clone())
            .service(health::health)
        })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
