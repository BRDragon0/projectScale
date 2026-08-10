use std::fs;

pub struct ObjectData{
    pub verticies: Vec<f32>,
    pub faces: Vec<u16>,
    pub uv: Vec<f32>,
    pub normals: Vec<f32>,
}

pub fn getObjectData(filePath: String) -> ObjectData{
    let rawData: String = fs::read_to_string(format!("../assets/objects/{filePath}.obj")).unwrap();
    let formatedData: Vec<&str> = rawData.split('\n').collect();
    let mut vc: Vec<f32> = Vec::new();
    let mut nm: Vec<f32> = Vec::new();
    let mut uv: Vec<f32> = Vec::new();
    let mut fc: Vec<u16> = Vec::new();
    let mut data: Vec<Vec<&str>> = Vec::new();
    for n in 0..formatedData.len(){
        data.push(formatedData[n].split(' ').collect());
    }
    for n in 0..data.len(){
        match data[n][0] {
            "v" => {
                vc.push(data[n][1].parse().unwrap());
                vc.push(data[n][2].parse().unwrap());
                vc.push(data[n][3].parse().unwrap());
            },
            "vn" => {
                nm.push(data[n][1].parse().unwrap());
                nm.push(data[n][2].parse().unwrap());
                nm.push(data[n][3].parse().unwrap());
            },
            "vt" => {
                uv.push(data[n][1].parse().unwrap());
                uv.push(data[n][2].parse().unwrap());
            },
            "f" => {
                for i in 1..4{
                    let tempSplit: Vec<&str> = data[n][i].split('/').collect();
                    fc.push(tempSplit[0].parse().unwrap());
                    fc.push(tempSplit[1].parse().unwrap());
                    fc.push(tempSplit[2].parse().unwrap());
                }
            },
            _ => {
                println!("Error");
            }
        }
    }
    ObjectData{ verticies: vc, faces: fc, uv: uv, normals: nm }
}