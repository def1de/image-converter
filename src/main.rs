#![windows_subsystem = "windows"]
#![allow(unused)]

mod img_manager;
use img_manager::{convert_image, is_image_extension};

use iced::widget::{button, column, text, Column};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum Format {
    PNG,
}

struct Converter {
    result: String,
    file: File,
}

impl Converter {
    fn view(&self) -> Column<Format> {
        let title = text("Choose a file extention");
        let btn_png = button(".PNG").on_press(Format::PNG);
        let debug = text(&self.result);

        let interface = column![title, btn_png, debug];
        interface
    }

    fn update(&mut self, format: Format) {
        self.file = get_file_path();
        match format {
            Format::PNG => match convert_image(self.file.path.clone(), ".png") {
                Ok(_) => {
                    self.result = "Image converted successfully".to_string();
                    std::process::exit(0);
                }
                Err(e) => self.result = format!("Error converting image: {:?}", e),
            },
        }
    }
}

impl Default for Converter {
    fn default() -> Self {
        Converter {
            result: String::new(),
            file: File {
                path: String::new(),
                ext: String::new(),
            },
        }
    }
}

struct File {
    path: String,
    ext: String,
}

fn get_file_path() -> File {
    // Get the input file path
    let args: Vec<String> = std::env::args().collect();

    let input_file = match args.get(1) {
        Some(file) => file.to_string(),
        None => {
            println!("Please provide an input file path");
            std::process::exit(1);
        }
    };
    let input_file = input_file.trim();

    // Get the extension of the input file
    let ext: &str = match input_file.split(".").collect::<Vec<&str>>().last() {
        Some(e) => e,
        None => {
            println!("No file extension found");
            std::process::exit(1);
        }
    };

    return File {
        path: input_file.to_string(),
        ext: ext.to_string(),
    };
}

fn app() -> iced::Result {
    iced::run("A Cool Title", Converter::update, Converter::view)
}

fn main() {
    let file = get_file_path();
    if is_image_extension(file.ext) {
        app();
        // match convert_image(file.path, ".png") {
        //     Ok(_) => println!("Image converted successfully"),
        //     Err(e) => println!("Error converting image: {:?}", e),
        // }
    } else {
        println!("Invalid image file extension");
    }
}
