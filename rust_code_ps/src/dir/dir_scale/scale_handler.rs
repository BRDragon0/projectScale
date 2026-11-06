use std::fs;
use std::collections::HashMap;

use regex::Regex;

pub struct Scale{
    _id: String,
    _base: String,
    _object: String,
    _texture: String,
    //Measured in pixels
    _pivot:  [i32; 3],
    //Always 3D but 2D objects z is fixed
    _position: [f32; 3],
    _scale: [f32; 3],
    //Converted to quaterions
    _rotation: [f32; 4],
    //LINK - ACTION MAY BE CHANGED LATER
    _actions: Vec<String>,
}

pub fn get_scale_data(file_name: &str){
    //Gets data from scale file and returns Vecs holding data types
    let raw_data: String = fs::read_to_string(format!("scale_files/{file_name}.scale")).unwrap_or_else(|err| {
        println!("Error finding file: {}", err);
        "fallback".to_string()
    });
    //Removes comments
    let rx_comms = Regex::new(r"(?s:/{3}.+/{3})|()/{2}.*\n+").unwrap();
    let mut cleaned_data = rx_comms.replace_all(&raw_data, "").into_owned();
    //Clean data (removing not needed whitespace) using regex
    let rx_clean = Regex::new(r" +|\t+|\r+|\n+").unwrap();
    cleaned_data = rx_clean.replace_all(&cleaned_data, "").into_owned();
    //Split data into formatted vector so each character can be iterated over
    let formatted_data: Vec<&str> = cleaned_data.split("").collect();

    //Anything from ! or ~ will go into dictonary and anything from + will go into scene
    //String is ID, scale is scale object
    //Contains data of objects in the scene
    let mut _scene:  HashMap<String, Scale> = HashMap::new();
    //String is Name, scale is scale object
    //Contains data of possible objects as a dictonary
    let mut _dictonary:  HashMap<String, Scale> = HashMap::new();

    //In this circumstance it would be better to use a while loop
    //But I dont like while loops as my stupidity could lead to infinite loops
    //So I will be using a for loop with a break instead with it looping over the uper bound
    for _c in 0..formatted_data.len(){
    
    }


}