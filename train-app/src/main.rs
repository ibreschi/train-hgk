#[macro_use]
extern crate actix_web;

use std::{io};

use actix_web::{middleware, App, HttpServer};

// mod urls;
mod constants;
mod health;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");
    // Set up DB pool to be used with web::Data<Pool> extractor
    // app.data(pool.clone());
    // // enable logger - always register actix-web Logger middleware last

    // urls::register_services(app);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health::health)
        })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
