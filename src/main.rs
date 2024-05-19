use std::fs;
use std::path::Path;

// fetch all teh files with .jpg extension from the specified directory
fn fetch_jpg_files(folder_path: &str) -> Vec<String> {
    let mut jpg_files = Vec::new();
    let folder = Path::new(folder_path);

    if folder.is_dir() {
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if let Some(extension) = file_path.extension() {
                        if extension == "jpg" {
                            if let Some(file_name) = file_path.to_str() {
                                jpg_files.push(file_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    return jpg_files;
}

fn convert_jpg_to_png(jpg_files: Vec<String>, png_folder_path: &str) {
    match fs::create_dir(&png_folder_path) {
        // Create a new folder
        Ok(_) => println!("Folder created successfully"),
        Err(e) => println!("Error creating folder: {}", e),
    }

    for jpg_file in jpg_files {
        println!("Converting \n\t {}\nto .png", jpg_file);
        let png_file_path: &str = jpg_file.split(".").collect::<Vec<&str>>()[0];
        let png_file_path_parts = png_file_path.split("\\").collect::<Vec<&str>>();
        let png_image_name = png_file_path_parts.last().unwrap();
        let png_name = png_image_name.to_owned().to_string() + ".png";
        let png_path = png_folder_path.to_owned() + "\\" + png_name.as_str();

        match image::open(&jpg_file) {
            Ok(img) => {
                let img = img.to_rgba8();
                match img.save(png_path) {
                    Ok(_) => println!("Image saved successfully"),
                    Err(e) => println!("Error saving image: {}", e),
                }
            }
            Err(e) => {
                println!("Error opening image: {}", e);
                continue;
            }
        };
    }
}

fn main() {
    let folder_path = "C:\\Users\\iliak\\Desktop\\";
    let png_folder_path = folder_path.to_owned() + "png";

    let jpg_files = fetch_jpg_files(folder_path);

    convert_jpg_to_png(jpg_files, &png_folder_path);
}
