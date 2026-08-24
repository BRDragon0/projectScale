use std::fs;

pub fn get_scale_data(file_name: &str){
    let raw_data: String = fs::read_to_string(format!("scale_files/{file_name}.scale")).unwrap_or_else(|err| {
        println!("Error finding file: {}", err);
        "fallback".to_string()
    });
}