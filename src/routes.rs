use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use validator::Validate;

use crate::data;
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

#[get("/test")]
pub async fn test(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let qr_svg = manager::check_qr_code("", "");
    let data = json!({
                "title" : "QR Lost Things Test",
                "parent" : "template",
                "qr": qr_svg,
            });
    let body = hb.render("done", &data).unwrap();
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
            let cid = manager::send_confirm_code(&email).unwrap();
            let data = json!({
                "title" : "QR Lost Things",
                "parent" : "template",
                "cid": cid,
                "email" : &email,
            });
            hb.render("confirm", &data).unwrap()
        }
        Err(e) => {
            warn!("😀 just logging an err with err: {}", e);

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
            let qr_svg = manager::check_qr_code(&confirm_form.cid, &confirm_form.code);
            let data = json!({
                "title" : "QR Lost Things",
                "parent" : "template",
                "qr": qr_svg,
            });
            hb.render("done", &data).unwrap()
        }
        Err(e) => {
            warn!("err with confirm code: {}", e);
            let data = json!({
                "error": "Confirm code is invalid",
            });
            hb.render("confirm", &data).unwrap()
        }
    };
    HttpResponse::Ok().body(body)
}
