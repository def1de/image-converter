use image;
use std::io::Error;
use std::sync::{Arc, Mutex};

const IMAGES: [&str; 11] = [
    "jpg", "avif", "bmp", "gif", "png", "pnm", "qoi", "tga", "tiff", "webp", "ico",
];

pub fn is_image_extension(file_ext: String) -> bool {
    IMAGES.contains(&file_ext.to_lowercase().as_str())
}

pub fn convert_image(input_file: String, ext: &str) -> Result<(), Error> {
    // Get the name of the image file
    println!("Converting image");
    let new_file_path: &str = input_file.split(".").collect::<Vec<&str>>()[0];
    let new_path = new_file_path.to_owned() + ext;

    // Open the image file
    match image::open(&input_file) {
        Ok(img) => {
            let img = img.to_rgba8();
            // Save the image file with required extension
            match img.save(new_path) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error reading image file {}", input_file),
                    ));
                }
            }
        }
        Err(e) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Error opening image file: {}", input_file),
            ));
        }
    };
}
