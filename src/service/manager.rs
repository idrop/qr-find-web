use uuid::Uuid;

pub fn send_confirm_code(_email: &str) -> Result<String, String> {
    let cid = Uuid::new_v4().to_string();
    // todo sqs pub send email and cid, with an expiry of 20 mins
    // todo lambda sub generate confirm code, and email to email address
    // todo sqs pub confirm code & email on topic "cid"
    Ok(cid)
}

pub fn check_qr_code(_cid: &str, _confirm_code: &str) -> Result<(), String> {
    // todo sqs sub topic "cid" to retrieve confirm code
    // todo compare topic confirm code with given confirm_code

    Ok(())
}
