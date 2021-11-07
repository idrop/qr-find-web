extern crate qrcodegen;

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

const BORDER: i32 = 4;
const SPACE: &str = " ";

pub fn get_qr_path_d(url_prefix: &str, id: &str) -> String {
    let url = &[url_prefix, id].concat();
    // Make and print the QR Code symbol
    let qr: QrCode = QrCode::encode_text(url, QrCodeEcc::High).unwrap();
    // to_svg_string(&qr, 4)
    get_svg_path_d(qr, BORDER)
}

fn get_svg_path_d(qr: QrCode, border: i32) -> String {
    let mut v: Vec<String> = Vec::new();
    let qr_size: i32 = qr.size();
    info!("qr width={}", qr_size);
    for y in 0..qr_size {
        for x in 0..qr_size {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    v.push(String::from(SPACE));
                }
                v.push(format!("M{},{}h1v1h-1z", x + border, y + border));
            }
        }
    }
    v.concat()
}
