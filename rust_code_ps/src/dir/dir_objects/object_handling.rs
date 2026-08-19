use glam::{Mat4, Vec3, Quat, EulerRot};
use crate::graphics::Vertex;
use std::collections::HashMap;

use crate::dir::dir_objects::object_parser::{ObjectData, get_object_data};

pub struct Object{
    transform: Mat4,
    faces: Vec<[u16; 3]>,
    verticies: Vec<[f32; 3]>,
    uv: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
}

impl Object{
    pub fn new(object_type: &str) -> Object{
    let object_data: ObjectData = get_object_data(object_type);
        Object {
            transform: Mat4::from_cols_array_2d(&[
            [1.0, 0.0, 0.0, 0.0,],
            [0.0, 1.0, 0.0, 0.0,],
            [0.0, 0.0, 1.0, 0.0,],
            [0.0, 0.0, 0.0, 1.0,],
            ]),
            verticies: object_data.verticies,
            normals: object_data.normals,
            uv: object_data.uv,
            faces: object_data.faces,
        }
    }
}

pub trait ObjectStates {
    fn set_object_state(&mut self, position: Vec3, rotation: Vec3, scale: Vec3);
    fn set_object_position(&mut self, position: Vec3);
    fn set_object_rotation(&mut self, rotation: Vec3);
    fn set_object_scale(&mut self, scale: Vec3);

    fn add_object_state(&mut self, position: Vec3, rotation: Vec3, scale: Vec3);
    fn add_object_position(&mut self, position: Vec3);
    fn add_object_rotation(&mut self, rotation: Vec3);
    fn add_object_scale(&mut self, scale: Vec3);

    fn get_object_data(&self) -> (Mat4, Vec<[u16; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 3]>);
    fn get_object_state(&self) -> Mat4;
    fn get_object_face(&self) -> Vec<[u16; 3]>;
    fn get_object_vert(&self) -> Vec<[f32; 3]>;
    fn get_object_uv(&self) -> Vec<[f32; 2]>;
    fn get_object_norm(&self) -> Vec<[f32; 3]>;

    fn format_object(&self) -> (Vec<u16>, Vec<Vertex>);
}

impl ObjectStates for Object{
    //SECTION - SET SECTION
    fn set_object_state(&mut self, position: Vec3, rotation: Vec3, scale: Vec3){
        self.transform = Mat4::from_scale_rotation_translation(
            scale,
            Quat::from_euler(
                EulerRot::XYZ, 
                rotation[0].to_radians(), 
                rotation[1].to_radians(), 
                rotation[2].to_radians(),
            ), 
            position,
        );
    }
    fn set_object_position(&mut self, position: Vec3) {
        self.transform = Mat4::from_scale_rotation_translation(
            self.transform.to_scale_rotation_translation().0,
            self.transform.to_scale_rotation_translation().1,
            position,
        );
    }
    fn set_object_rotation(&mut self, rotation: Vec3) {
        self.transform = Mat4::from_scale_rotation_translation(
            self.transform.to_scale_rotation_translation().0,
            Quat::from_euler(
                EulerRot::XYZ, 
                rotation[0].to_radians(), 
                rotation[1].to_radians(), 
                rotation[2].to_radians(),
            ), 
            self.transform.to_scale_rotation_translation().2,
        );
    }
    fn set_object_scale(&mut self, scale: Vec3) {
        self.transform = Mat4::from_scale_rotation_translation(
            scale,
            self.transform.to_scale_rotation_translation().1,
            self.transform.to_scale_rotation_translation().2,
        );
    }
    //SECTION - ADD SECTION
    fn add_object_state(&mut self, position: Vec3, rotation: Vec3, scale: Vec3){
        self.transform = Mat4::from_scale_rotation_translation(
            scale,
            Quat::from_euler(
                EulerRot::XYZ, 
                rotation[0].to_radians(), 
                rotation[1].to_radians(), 
                rotation[2].to_radians(),
            ), 
            position,
        ) + self.transform;
    }
    fn add_object_position(&mut self, position: Vec3) {
        self.transform = Mat4::from_translation(position) + self.transform;
    }
    fn add_object_rotation(&mut self, rotation: Vec3) {
        self.transform = Mat4::from_euler(
            EulerRot::XYZ, 
            rotation[0].to_radians(), 
            rotation[1].to_radians(), 
            rotation[2].to_radians(),
        ) + self.transform
    }
    fn add_object_scale(&mut self, scale: Vec3) {
        self.transform = Mat4::from_scale(scale) + self.transform;
    }
    //SECTION - GET SECTION
    fn get_object_data(&self) -> (Mat4, Vec<[u16; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 3]>) {
        (
            self.transform,
            self.faces.clone(),
            self.verticies.clone(),
            self.uv.clone(),
            self.normals.clone(),
        )
    }
    fn get_object_state(&self) -> Mat4 {
        self.transform
    }
    fn get_object_face(&self) -> Vec<[u16; 3]> {
        self.faces.clone()
    }
    fn get_object_vert(&self) -> Vec<[f32; 3]> {
        self.verticies.clone()
    }
    fn get_object_uv(&self) -> Vec<[f32; 2]> {
        self.uv.clone()
    }
    fn get_object_norm(&self) -> Vec<[f32; 3]> {
        self.normals.clone()
    }

    fn format_object(&self) -> (Vec<u16>, Vec<Vertex>){

            let mut dupe_map: HashMap<[u16; 3], u16> = HashMap::new();
            let mut vertices: Vec<Vertex> = Vec::new(); 
            let mut indices: Vec<u16> = Vec::new();

            let data = &self.get_object_data();
            let faces = &data.1;
            for n in 0..faces.len(){
                if dupe_map.get(&faces[n]) != None{
                    indices.push(*dupe_map.get(&faces[n]).unwrap());
                }
                else{
                    dupe_map.insert(faces[n], vertices.len() as u16);
                    vertices.push(Vertex{ 
                            position: data.2[(faces[n][0] - 1) as usize], 
                            uv: data.3[(faces[n][1] - 1) as usize], 
                            normals: data.4[(faces[n][2] - 1) as usize],
                    });
                    indices.push((vertices.len() - 1) as u16);
                }
            }
            if indices.len() % 2 == 1 {
                indices.push(0);
            }

            (indices, vertices)
    }
}