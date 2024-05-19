use colored::Colorize;
use std::fs;
use std::io;
use std::path::Path;

// fetch all teh files with .jpg extension from the specified directory
fn fetch_jpg_files(folder_path: &str) -> Vec<String> {
    let mut jpg_files = Vec::new();
    let folder = Path::new(folder_path);

    if folder.is_dir() {
        match fs::read_dir(folder) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let file_path = entry.path();
                            match file_path.extension() {
                                Some(extension) if extension == "jpg" => match file_path.to_str() {
                                    Some(file_name) => jpg_files.push(file_name.to_string()),
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                        Err(e) => println!("Error reading directory: {}", e),
                    }
                }
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    }

    match jpg_files.len() {
        0 => println!("No jpg files found"),
        _ => println!("{} jpg files found", jpg_files.len()),
    }

    return jpg_files;
}

// Open the folder in the file explorer
fn open_in_folder(folder_path: &str) {
    if cfg!(target_os = "windows") {
        if let Err(e) = std::process::Command::new("explorer")
            .arg(folder_path)
            .spawn()
        {
            println!("Error opening folder: {}", e);
        }
    } else if cfg!(target_os = "macos") {
        if let Err(e) = std::process::Command::new("open").arg(folder_path).spawn() {
            println!("Error opening folder: {}", e);
        }
    } else if cfg!(target_os = "linux") {
        if let Err(e) = std::process::Command::new("xdg-open")
            .arg(folder_path)
            .spawn()
        {
            println!("Error opening folder: {}", e);
        }
    } else {
        println!("Unsupported operating system");
    }
}

fn convert_jpg_to_png(jpg_files: Vec<String>, png_folder_path: &str) {
    match fs::create_dir(&png_folder_path) {
        // Create a new folder to store the converted images
        Ok(_) => println!("Folder created successfully"),
        Err(e) => println!("Error creating folder: {}", e),
    }

    for jpg_file in jpg_files {
        println!("Converting \n\t {}\nto .png", jpg_file.cyan());

        // Get the name of the image file
        let png_file_path: &str = jpg_file.split(".").collect::<Vec<&str>>()[0];
        let png_file_path_parts = png_file_path.split("\\").collect::<Vec<&str>>();
        let png_image_name = png_file_path_parts.last().unwrap();
        let png_name = png_image_name.to_owned().to_string() + ".png";
        let png_path = png_folder_path.to_owned() + "\\" + png_name.as_str();

        // Open the image file
        match image::open(&jpg_file) {
            Ok(img) => {
                let img = img.to_rgba8();
                // Save the image file with required extension
                match img.save(png_path) {
                    Ok(_) => println!("Image saved successfully\n"),
                    Err(e) => println!("Error saving image: {}", e),
                }
            }
            Err(e) => {
                println!("Error opening image: {}", e);
                continue;
            }
        };
    }
    println!("{}", "Conversion completed successfully\n".green());

    open_in_folder(png_folder_path);
}

fn main() {
    let mut folder_path = String::new();
    println!("Enter the folder path:");
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read input");
    let folder_path = folder_path.trim();

    let png_folder_path = folder_path.trim().to_owned() + "converted";

    let jpg_files = fetch_jpg_files(&folder_path);

    convert_jpg_to_png(jpg_files, &png_folder_path);
}
