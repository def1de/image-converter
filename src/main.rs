#![windows_subsystem = "windows"]
#![allow(unused)]

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{Box, Orientation};

fn convert_image(input_file: String, ext: &str) {
    // Get the name of the image file
    let new_file_path: &str = input_file.split(".").collect::<Vec<&str>>()[0];
    let new_path = new_file_path.to_owned() + ext;

    // Open the image file
    match image::open(&input_file) {
        Ok(img) => {
            let img = img.to_rgba8();
            // Save the image file with required extension
            match img.save(new_path) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => {}
    };
}

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

        for i in 1..10 {
            let button_label = format!("Button {}", i);
            let button = gtk::Button::builder().label(&button_label).margin_bottom(12).build();
            button.connect_clicked(move |_| {
                println!("Clicked! {}", &button_label);
            });
            vbox.append(&button);
        }

        window.present();
    });

    application.run()
}
