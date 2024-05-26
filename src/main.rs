#![windows_subsystem = "windows"]
#![allow(unused)]

mod img_manager;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, gio::ApplicationFlags};
use gtk::{Box, Orientation};

struct File{
    path: String,
    ext: String
}

fn get_file_path() -> File {
    // Get the input file path
    let args: Vec<String> = std::env::args().collect();
    
    let input_file = args[1].to_string();
    let input_file = input_file.trim();

    //Get the extension of the input file
    let ext: &str = match input_file.split(".").collect::<Vec<&str>>().last(){
        Some(e) => e,
        None => ""
    };

    return File{
        path: input_file.to_string(),
        ext: ext.to_string()
    }
}

fn main() -> glib::ExitCode{
    
    let application = Application::builder()
        .application_id("com.def1de.CrossConvert")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    application.connect_open(|app, files, _| {
        // Get the input file path
        let file = get_file_path();

        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cross Convert")
            .default_width(350)
            .default_height(70)
            .build();

        // Create a vertical box layout
        let vbox = Box::builder().orientation(Orientation::Vertical)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
        window.set_child(Some(&vbox));

        let title_text = gtk::Label::builder()
            .label("Select the format to convert the image to:")
            .margin_bottom(12)
            .build();
        vbox.append(&title_text);

        // Check if the input file is a supported file format
        if img_manager::is_image_extension(file.ext) {
            img_manager::create_app_buttons(&vbox, file.path.clone());
        } else {
            let error_text = gtk::Label::builder()
                .label("This file format is not supported")
                .margin_bottom(12)
                .build();
            vbox.append(&error_text);
        }

        window.present();
    });

    application.run_with_args(&std::env::args().collect::<Vec<_>>())
}