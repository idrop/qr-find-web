use serde::Deserialize;

use lazy_static::lazy_static;
use regex::Regex;
use validator::Validate;

lazy_static! {
    static ref RE_CONFIRM_CODE: Regex = Regex::new(r"^[0-9]{6}$").unwrap();
    static ref RE_UUID_4: Regex = Regex::new(
        r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$"
    )
    .unwrap();
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
}
