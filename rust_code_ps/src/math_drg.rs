use std::f64::consts::PI;
use glam::{Mat4, Vec3, Vec4, Quat, EulerRot};

pub fn to_rads(deg: f32) -> f32{
    let rad: f32 = deg * (PI as f32 / 180.0);
    rad
}