use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

lazy_static! {
    static ref RE_CONFIRM_CODE: Regex = Regex::new(r"^[0-9]{6}$").unwrap();
    static ref RE_UUID_4: Regex = Regex::new(r"[A-Za-z0-9-_~]{10}$").unwrap();
}

#[derive(Debug, Validate, Deserialize)]
pub struct EmailForm {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ConfirmForm {
    #[validate(regex = "crate::data::RE_CONFIRM_CODE")]
    pub code: String,
    #[validate(regex = "crate::data::RE_UUID_4")]
    pub cid: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct PrintForm {
    #[validate(regex = "crate::data::RE_UUID_4")]
    pub cid: String,
}

#[derive(Deserialize, Serialize)]
pub struct PrintPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Serialize)]
pub struct PrintLayout {
    pub qr_svg_path_d: String,
    pub positions: Vec<PrintPosition>,
    pub x_max: i32,
    pub y_max: i32,
}
