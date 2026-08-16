use std::fs;

pub struct ObjectData{
    pub faces: Vec<[u16; 3]>,
    pub verticies: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
}

pub fn getObjectData(filePath: &str) -> ObjectData{
    let rawData: String = fs::read_to_string(format!("../assets/objects/{filePath}.obj")).unwrap();
    let formatedData: Vec<&str> = rawData.split('\n').collect();
    let mut fc: Vec<[u16; 3]> = Vec::new();
    let mut vc: Vec<[f32; 3]> = Vec::new();
    let mut uv: Vec<[f32; 2]> = Vec::new();
    let mut nm: Vec<[f32; 3]> = Vec::new();
    let mut data: Vec<Vec<&str>> = Vec::new();
    for n in 0..formatedData.len(){
        data.push(formatedData[n].split(' ').collect());
    }
    for n in 0..data.len(){
        match data[n][0] {
            "f" => {
                for i in 1..4{
                    let tempSplit: Vec<&str> = data[n][i].split('/').collect();
                    fc.push([
                        tempSplit[0].parse::<u16>().unwrap(),
                        tempSplit[1].parse::<u16>().unwrap(),
                        tempSplit[2].parse::<u16>().unwrap(),
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
                data[n][2].parse::<f32>().unwrap(),
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