



mod qr_code {

    extern crate qrcodegen;

    use qrcodegen::Mask;
    use qrcodegen::QrCode;
    use qrcodegen::QrCodeEcc;
    use qrcodegen::QrSegment;
    use qrcodegen::Version;

    // Creates a single QR Code, then prints it to the console.
    pub fn do_basic_demo() {
        let text: &'static str = "https://seven.cn/A3tUiu/01itesttest";   // User-supplied Unicode text
        // Error correction level

        // Make and print the QR Code symbol
        let qr: QrCode = QrCode::encode_text(text, QrCodeEcc::High).unwrap();
        print_qr(&qr);
        println!("{}", to_svg_string(&qr, 4));
    }


    // Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform.
    fn to_svg_string(qr: &QrCode, border: i32) -> String {
        assert!(border >= 0, "Border must be non-negative");
        let mut result = String::new();
        result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
        result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
        let dimension = qr.size().checked_add(border.checked_mul(2).unwrap()).unwrap();
        result += &format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
        result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
        result += "\t<path d=\"";
        for y in 0..qr.size() {
            for x in 0..qr.size() {
                if qr.get_module(x, y) {
                    if x != 0 || y != 0 {
                        result += " ";
                    }
                    result += &format!("M{},{}h1v1h-1z", x + border, y + border);
                }
            }
        }
        result += "\" fill=\"#000000\"/>\n";
        result += "</svg>\n";
        result
    }


}