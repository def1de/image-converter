#![windows_subsystem = "windows"]

fn convert_jpg_to_png(jpg_file: String) {
    // Get the name of the image file
    let png_file_path: &str = jpg_file.split(".").collect::<Vec<&str>>()[0];
    let png_path = png_file_path.to_owned() + ".png";

    // Open the image file
    match image::open(&jpg_file) {
        Ok(img) => {
            let img = img.to_rgba8();
            // Save the image file with required extension
            match img.save(png_path) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => {}
    };
}

fn main() {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();

    let jpg_file = args[1].to_string();
    let jpg_file = jpg_file.trim();

    convert_jpg_to_png(jpg_file.to_string());
}
