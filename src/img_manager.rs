use image;
use std::io::Error;

#[derive(Debug, Clone)]
pub enum Format {
    JPG,
    AVIF,
    BMP,
    GIF,
    PNG,
    QOI,
    TGA,
    TIFF,
    WEBP,
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Format::JPG => ".jpg",
            Format::AVIF => ".avif",
            Format::BMP => ".bmp",
            Format::GIF => ".gif",
            Format::PNG => ".png",
            Format::QOI => ".qoi",
            Format::TGA => ".tga",
            Format::TIFF => ".tiff",
            Format::WEBP => ".webp",
        }
    }

    pub fn from_str(format: &str) -> Option<Format> {
        match format {
            "jpg" => Some(Format::JPG),
            "avif" => Some(Format::AVIF),
            "bmp" => Some(Format::BMP),
            "gif" => Some(Format::GIF),
            "png" => Some(Format::PNG),
            "qoi" => Some(Format::QOI),
            "tga" => Some(Format::TGA),
            "tiff" => Some(Format::TIFF),
            "webp" => Some(Format::WEBP),
            _ => None,
        }
    }

    pub fn values() -> Vec<Format> {
        vec![
            Format::JPG,
            Format::AVIF,
            Format::BMP,
            Format::GIF,
            Format::PNG,
            Format::QOI,
            Format::TGA,
            Format::TIFF,
            Format::WEBP,
        ]
    }
}

pub fn is_image_extension(file_ext: &String) -> bool {
    let format = Format::from_str(file_ext);
    match format {
        Some(_) => true,
        None => false,
    }
}

pub fn convert_image(input_file: String, ext: &str) -> Result<(), Error> {
    // Get the name of the image file
    println!("Converting image");
    let new_file_path: &str = input_file.split(".").collect::<Vec<&str>>()[0];
    let new_path = new_file_path.to_owned() + ext;

    // Open the image file
    match image::open(&input_file) {
        Ok(img) => {
            let img = img.to_rgb8();
            // Save the image file with required extension
            match img.save(new_path) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error reading image file {}\n{}", input_file, e),
                    ));
                }
            }
        }
        Err(e) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Error opening image file: {}\n{}", input_file, e),
            ));
        }
    };
}
