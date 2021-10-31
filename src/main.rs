#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use std::io;

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use handlebars::Handlebars;

mod data;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./web/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .wrap(error_handlers())
            .app_data(handlebars_ref.clone())
            .service(Files::new("/assets", "web/templates/assets/"))
            .service(routes::index)
            .service(routes::index_post)
            .service(routes::confirm_post)
    })
    .workers(3)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
