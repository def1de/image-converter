#![windows_subsystem = "windows"]
#![allow(unused)]

mod img_manager;
mod theme;

use iced::widget::shader::wgpu::naga::proc;
use img_manager::{convert_image, is_image_extension, Format};

use iced::executor;
use iced::widget::{button, container, text, Button, Column, Container, Row, Space, Text};
use iced::{window, Application, Command, Settings};
use theme::Theme;

pub type Element<'a> = iced::Element<'a, img_manager::Format, theme::Theme, iced::Renderer>;

#[derive(Debug, Clone)]
struct App {
    result: String,
    file: File,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Format;
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (
            App {
                result: String::new(),
                file: get_file_path(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn view(&self) -> Element {
        let close_btn =
            Button::new(Text::new("x").horizontal_alignment(iced::alignment::Horizontal::Center))
                .on_press(Format::None)
                .style(theme::Button::Close)
                .width(35)
                .height(35)
                .padding(5);

        let row = Row::new()
            .push(Space::with_width(iced::Length::Fill))
            .push(close_btn);

        let mut column: Column<'_, Format, Theme> = Column::new();

        let title: Text<'_, Theme, _> = text("Choose a file extention");
        column = column.push(title);

        // Create a button for each format
        for format in Format::values() {
            if format == Format::None {
                continue;
            }
            let btn: Button<'_, Format, Theme, _> = Button::new(
                Text::new(format.as_str())
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .on_press(format)
            .style(theme::Button::Primary)
            .width(100)
            .padding(5);
            column = column.push(btn);
        }

        let debug: Text<'_, Theme, _> = text(&self.result);
        column = column.push(debug);
        column = column.align_items(iced::Alignment::Center).spacing(10);

        let content = Column::new()
            .push(row)
            .push(column)
            .align_items(iced::Alignment::Center);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .into()
    }

    fn update(&mut self, format: Self::Message) -> Command<Self::Message> {
        self.file = get_file_path();
        if !is_image_extension(&self.file.ext) {
            self.result = "Invalid file extension".to_string();
            return Command::none();
        }
        if format == Format::None {
            std::process::exit(0);
        }
        match convert_image(self.file.path.clone(), format.as_str()) {
            Ok(_) => {
                self.result = "Image converted successfully".to_string();
                std::process::exit(0);
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
    let settings = Settings {
        window: window::Settings {
            size: iced::Size::new(300.0, 500.0),
            resizable: false,
            decorations: false,
            ..Default::default()
        },
        ..Default::default()
    };

    App::run(settings)
}

fn main() {
    let _ = start_app();
}
