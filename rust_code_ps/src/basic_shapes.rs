use rand::Rng;
use glam::Vec3;

pub struct Basic_Shape{
    pub identity: String,
    //X Y Z
    pub position: Vec3,
    //P Q R
    pub rotation: Vec3,
    //A B C
    pub dimentions: [f32; 3],
    pub verticies: Vec<f32>,
    pub faces: [u16],
}

impl Basic_Shape{
    pub fn new(identity: String, dimensions: Vec3, position: Vec3, rotation: Vec3) -> Self{
        //DirectionSetter
        const DS: [f32; 2] = [-1.0, 1.0];
        let mut verticies: Vec<f32> = Vec::new(); 
        match identity.as_str(){
            //SECTION - CUBEIOD
            "cubiod" => {
                for x in 0..2{
                    for y in 0..2{
                        for z in 0..2{
                            verticies.extend(vec![DS[x], DS[y], DS[z], 1.0]);
                            //TODO - REMOVE AND REPLACE WITH UV
                            verticies.extend(vec![rand::random(), rand::random(), rand::random()]);
                        }
                    }
                }
            },
        }
    }
}