use actix_web::web::{Form};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use nanoid::nanoid;
use validator::Validate;

use crate::data;
use crate::data::{ConfirmForm, PrintPosition};
use crate::service::manager;

#[get("/healthcheck")]
pub async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "title" : "QR Lost Things",
        "parent" : "template",
    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/")]
pub async fn index_post(
    hb: web::Data<Handlebars<'_>>,
    email_form: web::Form<data::EmailForm>,
) -> HttpResponse {
    let body = match email_form.validate() {
        Ok(_) => {
            let email = &email_form.email;
            let cid = manager::send_confirm_code(email).unwrap();
            let data = json!({
                "title" : "QR Lost Things",
                "parent" : "template",
                "cid": cid,
                "email" : email,
            });
            hb.render("confirm", &data).unwrap()
        }
        Err(_) => {
            let data = json!({
                "title" : "QR Lost Things",
                "parent" : "template",
                "email": &email_form.email,
                "email-error": true,
            });

            hb.render("index", &data).unwrap()
        }
    };
    HttpResponse::Ok().body(body)
}

#[post("/confirm")]
pub async fn confirm_post(
    hb: web::Data<Handlebars<'_>>,
    confirm_form: web::Form<data::ConfirmForm>,
) -> HttpResponse {
    let body = match confirm_form.validate() {
        Ok(_) => {
            let ok = manager::check_confirm_code(&confirm_form.cid, &confirm_form.code);
            if !ok {
                confirm_error_render(hb, &confirm_form)
            } else {
                let qr_d = manager::get_qr_code_path_d("url", &confirm_form.cid).unwrap();
                let data = json!({
                    "title" : "QR Lost Things",
                    "parent" : "template",
                    "qr_svg_path_d": qr_d,
                });
                hb.render("done", &data).unwrap()
            }
        }
        Err(e) => {
            warn!("err with confirm code: {}", e);
            confirm_error_render(hb, &confirm_form)
        }
    };
    HttpResponse::Ok().body(body)
}

fn confirm_error_render(hb: web::Data<Handlebars<'_>>, confirm_form: &Form<ConfirmForm>) -> String {
    let data = json!({
        "title" : "QR Lost Things",
        "parent" : "template",
        "cid": &confirm_form.cid,
        "email" : &confirm_form.email,
        "code-error" : true
    });
    hb.render("confirm", &data).unwrap()
}

#[post("/print")]
pub async fn print(
    hb: web::Data<Handlebars<'_>>,
    print_form: web::Form<data::PrintForm>,
) -> HttpResponse {
    let layout = manager::get_qr_print("https://short.url", &print_form.cid, 14);

    let data = json!({
        "qr_svg_path_d": layout.qr_svg_path_d,
        "positions": layout.positions,
        "x_max": layout.x_max,
        "y_max": layout.y_max,
    });
    let body = hb.render("print", &data).unwrap();
    HttpResponse::Ok().content_type("image/svg+xml").body(body)
}
