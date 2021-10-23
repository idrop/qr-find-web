#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use std::io;

use actix_files::Files;
use actix_http::{body::Body, Response};
use actix_web::{App, HttpResponse, HttpServer, Result, web};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use env_logger::Env;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use uuid::Uuid;
// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE2: Regex = Regex::new(r"^[0-9]{6}$").unwrap();
}

#[derive(Debug, Validate, Deserialize)]
pub struct EmailForm {
    #[validate(email)]
    email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ConfirmForm {
    #[validate(regex = "crate::RE2")]
    code: String,
}


// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "rid": "test"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/")]
async fn index_post(
    hb: web::Data<Handlebars<'_>>,
    email_form: web::Form<EmailForm>,
) -> HttpResponse {
    info!("😀 just logging an info");

    let data = json!({
        "cid": Uuid::new_v4().to_string()
    });

    match email_form.validate() {
        Ok(_) => {
            let body = hb.render("confirm", &data).unwrap();
            info!("😀 got email '{}'", email_form.email);
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            warn!("😀 just logging an err with err: {}", e);
            let body = hb.render("index", &data).unwrap();
            HttpResponse::Ok().body(body)
        }
    }
}

#[post("/confirm")]
async fn confirm(
    hb: web::Data<Handlebars<'_>>,
    confirm_form: web::Form<ConfirmForm>,
) -> HttpResponse {
    info!("😀 just logging an info on confirm form");

    let data = json!({
        "cid": Uuid::new_v4().to_string()
    });

    match confirm_form.validate() {
        Ok(_) => {
            HttpResponse::Ok()
                .content_type("image/svg+xml")
                .header("X-Hdr", "sample")
                .body("data")
        }
        Err(e) => {
            warn!("😀 just logging an err with err: {}", e);
            let body = hb.render("confirm", &data).unwrap();
            HttpResponse::Ok().body(body)
        }
    }
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //env_logger::from_env(Env::default().default_filter_or("info")).init();

    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
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
            .service(index)
            .service(index_post)
            .service(user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
