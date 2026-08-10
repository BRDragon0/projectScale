use rand::Rng;
use glam::{Mat4, Vec3};

use crate::obj_parser;

pub struct Object{
    pub transform: Mat4,
    verticies: Vec<f32>,
    faces: Vec<u16>,
    uv: Vec<f32>,
    normals: Vec<f32>,
}

impl Object{
    pub fn new(object_type: String) -> Self{
    let object_data: obj_parser::ObjectData = obj_parser::getObjectData(object_type);
        Self {
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