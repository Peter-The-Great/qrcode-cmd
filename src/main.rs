use image::Luma;
use qrcode::QrCode;
use std::env;

fn main() {
    // Get the command-line argument for the input text
    let args: Vec<String> = env::args().collect();

    // If there is no input text, print an error message and exit
    if args.len() < 2 {
        println!("Please provide the input text as an argument.");
        return;
    }
    let input_text = &args[1];

    // Generate the QR code
    let code = QrCode::new(input_text).expect("Failed to generate QR code.");

     // Encode some data into bits.
    let bitdata = QrCode::new(input_text).unwrap();

    // Render the bits into an image.
    let image = bitdata.render::<Luma<u8>>().build();

    // Save the image.
    image.save("qrcode.png").unwrap();

    // Render the QR code as ASCII
    let string = code.render::<char>().quiet_zone(false).module_dimensions(2, 1).build();

    // Render the bits into an image.
    println!("{}", string);
}