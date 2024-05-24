#![windows_subsystem = "windows"]

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

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
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
}
