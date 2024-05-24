#![windows_subsystem = "windows"]
#![allow(unused)]

mod img_manager;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{Box, Orientation};

fn main() -> glib::ExitCode{
    // Get the command line arguments
    // let args: Vec<String> = std::env::args().collect();

    // let input_file = args[1].to_string();
    // let input_file = input_file.trim();

    // let ext: &str = ".png";

    // convert_image(input_file.to_string(), ext);

    let application = Application::builder()
        .application_id("com.def1de.CrossConvert")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cross Convert")
            .default_width(350)
            .default_height(70)
            .build();

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

        img_manager::create_app_buttons(&vbox);

        window.present();
    });

    application.run()
}
