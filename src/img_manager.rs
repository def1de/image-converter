use gtk4 as gtk;
use gtk::prelude::*;
use image;
use std::io::Error;
use std::sync::{Arc, Mutex};

const IMAGES: [&str; 11] = [
    "jpg",
    "avif",
    "bmp",
    "gif",
    "png",
    "pnm",
    "qoi",
    "tga",
    "tiff",
    "webp",
    "ico"
];

pub fn is_image_extension(file_ext: String) -> bool {
    IMAGES.contains(&file_ext.to_lowercase().as_str())
}

pub fn create_app_buttons(vbox: &gtk::Box, input_file: String) -> Result<(), Error> {
    println!("Creating buttons");
    for i in 0..IMAGES.len() {
        let button_label = format!("Convert to .{}", IMAGES[i]);
        let button = gtk::Button::builder().label(&button_label).margin_bottom(12).build();
        let input = input_file.clone();
        let vbox_clone = vbox.clone();
        button.connect_clicked(move |_| {
            if let Err(_) = convert_image(input.clone(), &format!(".{}", IMAGES[i])){
                let label = gtk::Label::builder().label("Error Saving Image").build();
                vbox_clone.append(&label);
            } else {
                std::process::exit(0);
            }
        });
        vbox.append(&button);
    }
    Ok(())
}

fn convert_image(input_file: String, ext: &str) -> Result<(), Error>{
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
                Err(e) => {return Err(Error::new(std::io::ErrorKind::Other, "Error reading image file"));}
            }
        }
        Err(e) => {return Err(Error::new(std::io::ErrorKind::Other, "Error opening image file"));}
    };
}