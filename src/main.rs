#![windows_subsystem = "windows"]
#![allow(unused)]

mod img_manager;
use img_manager::{convert_image, is_image_extension};

use std::time::Duration;

struct File {
    path: String,
    ext: String,
}

fn get_file_path() -> File {
    // Get the input file path
    let args: Vec<String> = std::env::args().collect();

    let input_file = args[1].to_string();
    let input_file = input_file.trim();

    //Get the extension of the input file
    let ext: &str = match input_file.split(".").collect::<Vec<&str>>().last() {
        Some(e) => e,
        None => "",
    };

    return File {
        path: input_file.to_string(),
        ext: ext.to_string(),
    };
}

fn main() {
    let file = get_file_path();
    if is_image_extension(file.ext) {
        match convert_image(file.path, ".png") {
            Ok(_) => println!("Image converted successfully"),
            Err(e) => println!("Error converting image: {:?}", e),
        }
    } else {
        println!("Invalid image file extension");
    }
}
