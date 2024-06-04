#![windows_subsystem = "windows"]

mod img_manager;
use img_manager::{convert_image, is_image_extension, Format};

use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{button, text, Column, Container};

#[derive(Debug, Clone)]
struct App {
    result: String,
    file: File,
}

impl Application for App{
    type Executor = executor::Default;
    type Flags = ();
    type Message = Format;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App {
            result: String::new(),
            file: get_file_path(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn view(&self) -> Element<Self::Message> {
        let mut column = Column::new();

        let title = text("Choose a file extention");
        column = column.push(title);
        
        for format in Format::values() {
            let btn = button(format.as_str()).on_press(format);
            column = column.push(btn);
        }

        let debug = text(&self.result);
        column = column.push(debug);
        
        let element: Element<_, _> = Container::new(column).into();
        element
    }

    fn update(&mut self, format: Self::Message) -> Command<Self::Message> {
        self.file = get_file_path();
        if !is_image_extension(&self.file.ext) {
            self.result = "Invalid file extension".to_string();
            return Command::none();
        }
        match convert_image(self.file.path.clone(), format.as_str()) {
            Ok(_) => {
                self.result = "Image converted successfully".to_string();
            }
            Err(e) => {
                self.result = format!("Error converting image: {}", e);
            }
        }
        Command::none()
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            result: String::new(),
            file: File {
                path: String::new(),
                ext: String::new(),
            },
        }
    }
}

#[derive(Debug, Clone)]
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

fn start_app() -> iced::Result {
    App::run(Settings::default())
}

fn main() {
    let _ = start_app();
}