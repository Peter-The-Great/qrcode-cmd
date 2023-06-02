use qrcode::QrCode;
use std::env;

fn main() {
    // Get the command-line argument for the input text
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the input text as an argument.");
        return;
    }
    let input_text = &args[1];

    // Generate the QR code
    let code = QrCode::new(input_text).expect("Failed to generate QR code.");

    // Render the QR code as a SVG file
    let string = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
}