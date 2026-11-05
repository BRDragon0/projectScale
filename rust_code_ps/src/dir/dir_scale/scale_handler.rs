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
    //Clean data (removing not needed whitespace) using regex
    //raw_data.retain(|x| !['\t', '\n', '\r'].contains(&x));
    let rx_clean = Regex::new(r" *|\t*|\n*|\r*").unwrap();
    let cleaned_data = rx_clean.replace_all(&raw_data, "");
    let formatted_data: Vec<&str> = cleaned_data.split_inclusive(&['!', '+', '~', '#']).collect();
    //String is ID, scale is scale
    //Contains data of objects in the scene
    let mut _scene:  HashMap<String, Scale> = HashMap::new();
    //String is Name, scale is scale
    //Contains data of possible objects as a dictonary
    let mut _dictonary:  HashMap<String, Scale> = HashMap::new();

    println!("START");
    for def in 0..(formatted_data.len() / 2){
        println!("{}", formatted_data[def]);
        if formatted_data[def] == "!"{
            //Finds first { if not raise error
            let slice_pos = formatted_data[def].find('{');
            match slice_pos{
                Some(sp) => {
                    println!("got to here");
                    let id = &formatted_data[def][1..sp];
                    let data = &formatted_data[def][sp..formatted_data.len()];
                    println!("{}", id);
                    println!("{}", data);
                },
                None => {
                    println!("ERROR, no {{ found in file")
                },
            }

                // dictonary.insert(String::from(id), 
                //     Scale { 
                //         id: String::from(id),
                //         base: (), 
                //         object: (), 
                //         texture: (), 
                //         pivot: (), 
                //         position: (), 
                //         scale: (), 
                //         rotation: (), 
                //         actions: () 
                //     }
                // );
        }
    }
    println!("END");
}