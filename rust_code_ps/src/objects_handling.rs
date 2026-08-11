use rand::Rng;
use glam::{Mat4, Vec3, Quat, EulerRot};

use crate::obj_parser;

pub struct Object{
    transform: Mat4,
    verticies: Vec<f32>,
    faces: Vec<u16>,
    uv: Vec<f32>,
    normals: Vec<f32>,
}

impl Object{
    pub fn new(object_type: String) -> Object{
    let object_data: obj_parser::ObjectData = obj_parser::getObjectData(object_type);
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
    fn set_object_state(self, position: Vec3, rotation: Vec3, scale: Vec3);
    fn set_object_position(self, position: Vec3);
    fn set_object_rotation(self, rotation: Vec3);
    fn set_object_scale(self, scale: Vec3);

    fn add_object_state(self, position: Vec3, rotation: Vec3, scale: Vec3);
    fn add_object_position(self, position: Vec3);
    fn add_object_rotation(self, rotation: Vec3);
    fn add_object_scale(self, scale: Vec3);

    fn get_object_data(self) -> (Mat4, Vec<f32>, Vec<f32>, Vec<f32>, Vec<u16>);
    fn get_object_state(self) -> Mat4;
    fn get_object_vert(self) -> Vec<f32>;
    fn get_object_norm(self) -> Vec<f32>;
    fn get_object_uv(self) -> Vec<f32>;
    fn get_object_face(self) -> Vec<u16>;
}

impl ObjectStates for Object{
    //SECTION - SET SECTION
    fn set_object_state(mut self, position: Vec3, rotation: Vec3, scale: Vec3){
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
    fn set_object_position(mut self, position: Vec3) {
        self.transform = Mat4::from_scale_rotation_translation(
            self.transform.to_scale_rotation_translation().0,
            self.transform.to_scale_rotation_translation().1,
            position,
        );
    }
    fn set_object_rotation(mut self, rotation: Vec3) {
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
    fn set_object_scale(mut self, scale: Vec3) {
        self.transform = Mat4::from_scale_rotation_translation(
            scale,
            self.transform.to_scale_rotation_translation().1,
            self.transform.to_scale_rotation_translation().2,
        );
    }
    //SECTION - ADD SECTION
    fn add_object_state(mut self, position: Vec3, rotation: Vec3, scale: Vec3){
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
    fn add_object_position(mut self, position: Vec3) {
        self.transform = Mat4::from_translation(position) + self.transform;
    }
    fn add_object_rotation(mut self, rotation: Vec3) {
        self.transform = Mat4::from_euler(
            EulerRot::XYZ, 
            rotation[0].to_radians(), 
            rotation[1].to_radians(), 
            rotation[2].to_radians(),
        ) + self.transform
    }
    fn add_object_scale(mut self, scale: Vec3) {
        self.transform = Mat4::from_scale(scale) + self.transform;
    }
    //SECTION - GET SECTION
    fn get_object_data(self) -> (Mat4, Vec<f32>, Vec<f32>, Vec<f32>, Vec<u16>) {
        (
            self.transform,
            self.verticies,
            self.normals,
            self.uv,
            self.faces,
        )
    }
    fn get_object_state(self) -> Mat4 {
        self.transform
    }
    fn get_object_vert(self) -> Vec<f32> {
        self.verticies
    }
    fn get_object_norm(self) -> Vec<f32> {
        self.normals
    }
    fn get_object_uv(self) -> Vec<f32> {
        self.uv
    }
    fn get_object_face(self) -> Vec<u16> {
        self.faces
    }
}