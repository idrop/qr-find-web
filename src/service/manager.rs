use nanoid::nanoid;

use crate::data::{PrintLayout, PrintPosition};
use crate::service::qr;

pub fn send_confirm_code(_email: &str) -> Result<String, String> {
    let cid = nanoid!(10);
    // todo sqs pub send email and cid, with an expiry of 20 mins
    // todo lambda sub generate confirm code, and email it to email address
    // todo write confirm_code, cid, email to db
    Ok(cid)
}

pub fn check_confirm_code(_cid: &str, _confirm_code: &str) -> bool {
    // todo sqs sub topic "cid" to retrieve confirm code
    // todo compare topic confirm code with given confirm_code

    true
}

pub fn get_qr_code_path_d(url: &str, cid: &str) -> Result<String, String> {
    Ok(qr::get_qr_path_d(url, cid))
}

pub fn get_qr_print(url: &str, cid: &str, num_across: i32) -> PrintLayout {
    let qr_svg_path_d = get_qr_code_path_d(url, cid).unwrap();

    let mut positions: Vec<PrintPosition> = Vec::new();

    // A4 aspect ratio
    let num_down = (num_across as f32 * 1.4) as i32;

    for x in 0..num_across {
        for y in 0..num_down {
            positions.push(PrintPosition {
                x: 45 * x,
                y: 55 * y,
            });
        }
    }

    let x_max = positions.last().unwrap().x + 45;
    let y_max = positions.last().unwrap().y + 55;
    PrintLayout {
        qr_svg_path_d,
        positions,
        x_max,
        y_max,
    }
}
