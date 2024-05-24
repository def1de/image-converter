use gtk4 as gtk;
use gtk::prelude::*;
use image;

const IMAGES: [&str; 15] = [
    "jpg",
    "avif",
    "bmp",
    "dds",
    "gif",
    "hdr",
    "ico",
    "jpeg",
    "exr",
    "png",
    "pnm",
    "qoi",
    "tga",
    "tiff",
    "webp",
];

pub fn is_image_extension(file_ext: String) -> bool {
    IMAGES.contains(&file_ext.to_lowercase().as_str())
}

pub fn create_app_buttons(vbox: &gtk::Box, input_file: String) {
    println!("Creating buttons");
    for i in 0..IMAGES.len() {
        let button_label = format!("Convert to .{}", IMAGES[i]);
        let button = gtk::Button::builder().label(&button_label).margin_bottom(12).build();
        let input = input_file.clone();
        button.connect_clicked(move |_| {
            convert_image(input.clone(), &format!(".{}", IMAGES[i]));
        }); 
        vbox.append(&button);
    }
}

fn convert_image(input_file: String, ext: &str) {
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
                Ok(_) => {}
                Err(e) => {println!("Error saving image {}", e.to_string())}
            }
        }
        Err(_) => {println!("Error opening image")}
    };
}