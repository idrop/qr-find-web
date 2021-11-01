#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use std::env;
use std::io;

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
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
        .register_templates_directory(".hbs", "./web/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .wrap(error_handlers())
            .app_data(handlebars_ref.clone())
            .service(Files::new("/robots.txt", "web/templates/assets/robots.txt"))
            .service(Files::new("/assets", "web/templates/assets/"))
            .service(routes::index)
            .service(routes::index_post)
            .service(routes::confirm_post)
    })
    .workers(env::var("NUM_WORKERS").unwrap_or("3".to_string()).parse().unwrap())
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
