use glam::Vec3;

use crate::dir::dir_objects::object_handling::{ObjectStates, Object};

pub fn _act_rotate_to(rot: &[f32; 3], durration: &f32, mut parent: Object){
    let mut last_frame = std::time::Instant::now();
    let mut time = 0.0;
    //NOTE - WHILE LOOP HERE
    while time <= *durration{
        let dt =  std::time::Instant::now().duration_since(last_frame);
        last_frame = std::time::Instant::now();
        time += dt.as_secs_f32();
        parent.set_object_rotation(Vec3::new(rot[0] * time, rot[1] * time, rot[2] * time));
    }
}