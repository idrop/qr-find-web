extern crate qrcodegen;

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

const BORDER: i32 = 4;

// Creates a single QR Code, then prints it to the console.
pub fn get_qr_path_d(url_prefix: &str, id: &str) -> String {
    let url = &[url_prefix, id].concat();
    // Make and print the QR Code symbol
    let qr: QrCode = QrCode::encode_text(url, QrCodeEcc::High).unwrap();
    // to_svg_string(&qr, 4)
    get_svg_path_d(qr, BORDER)
}

// Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform.
fn to_svg_string(qr: QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let qr_size = qr.size();
    let dimension = qr_size.checked_add(border.checked_mul(2).unwrap()).unwrap();
    result += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += &*get_svg_path_d(qr, border);
    result += "</svg>\n";
    result
}

fn get_svg_path_d(qr: QrCode, border: i32) -> String {
    let mut v: Vec<String> = Vec::new();
    let qr_size: i32 = qr.size();

    for y in 0..qr_size {
        for x in 0..qr_size {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    v.push(String::from(" "));
                }
                v.push(format!("M{},{}h1v1h-1z", x + border, y + border));
            }
        }
    }
    v.concat()
}
