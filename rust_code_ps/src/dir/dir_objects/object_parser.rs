use std::fs;

use crate::dir::dir_fallbacks::fallback_defaults::FALLBACK_CUBE;

pub struct ObjectData{
    pub faces: Vec<[u16; 3]>,
    pub verticies: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
}

pub fn get_object_data(file_path: &str) -> ObjectData{
    let raw_data: String = fs::read_to_string(format!("assets/objects/{file_path}.obj")).unwrap_or_else(|err| {
        //Returns defualt cube if the file cant be found
        println!("Error finding file: {}", err);
        FALLBACK_CUBE.to_string()
    });
    let formated_data: Vec<&str> = raw_data.split('\n').collect();
    let mut fc: Vec<[u16; 3]> = Vec::new();
    let mut vc: Vec<[f32; 3]> = Vec::new();
    let mut uv: Vec<[f32; 2]> = Vec::new();
    let mut nm: Vec<[f32; 3]> = Vec::new();
    let mut data: Vec<Vec<&str>> = Vec::new();
    for n in 0..formated_data.len(){
        data.push(formated_data[n].split(' ').collect());
    }
    for n in 0..data.len(){
        match data[n][0] {
            "f" => {
                for i in 1..4{
                    let temp_split: Vec<&str> = data[n][i].split('/').collect();
                    fc.push([
                        temp_split[0].parse::<u16>().unwrap(),
                        temp_split[1].parse::<u16>().unwrap(),
                        temp_split[2].parse::<u16>().unwrap(),
                    ]);
                }
            },
            "v" => {vc.push([
                data[n][1].parse::<f32>().unwrap(),
                data[n][2].parse::<f32>().unwrap(),
                data[n][3].parse::<f32>().unwrap(),
            ]);},
            "vt" => {uv.push([
                data[n][1].parse::<f32>().unwrap(),
                1.0 - data[n][2].parse::<f32>().unwrap(),
            ]);},
            "vn" => {nm.push([
                data[n][1].parse::<f32>().unwrap(),
                data[n][2].parse::<f32>().unwrap(),
                data[n][3].parse::<f32>().unwrap(),
            ]);},
            _ => {
                
            }
        }
    }
    ObjectData{ verticies: vc, faces: fc, uv: uv, normals: nm }
}