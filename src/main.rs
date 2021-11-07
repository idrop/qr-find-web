#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use std::env;
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
    handlebars.set_dev_mode(true);
    handlebars
        .register_templates_directory(".hbs", "./web/templates")
        .expect("Couldn't register handlebars template dir");
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(handlebars_ref.clone())
            .service(Files::new("/robots.txt", "web/templates/assets/robots.txt"))
            .service(Files::new("/assets", "web/templates/assets/"))
            .service(routes::index)
            .service(routes::index_post)
            .service(routes::confirm_post)
            .service(routes::print)
            .service(routes::get_print)
            .service(routes::healthcheck)
    })
    .workers(num_workers())
    .bind(bind_address())?
    .run()
    .await
}

fn num_workers() -> usize {
    env::var("NUM_WORKERS")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .unwrap()
}

fn bind_address() -> String {
    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    [address, ":".to_string(), port].concat()
}
