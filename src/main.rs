use crate::util::qr_code::QRCodeEncoder;

mod util;

fn main() {
    let qr_code = QRCodeEncoder::new("HELLO", 1);

    println!("{:#?}", qr_code);
}
