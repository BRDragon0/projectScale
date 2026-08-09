use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3, Vec4, EulerRot};

#[repr(C)] // Makes sure fields in correct order
pub struct Camera{
    //x, y, z position
    pub position: [f32; 3],
    //pitch(p), yaw(q), roll(r)
    pub rotation: [f32; 3],
    //perspective or orthagonic
    pub perspective: bool,
    //Depending on perspective
    //False     Near, Far, Width, Hight
    //True      Near, Far, FOV (y), Aspect
    pub view_values: [f32; 4],
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct Uniform_Camera {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
}



pub trait CreateView {
    fn create_view(&self) -> Mat4;
}

impl CreateView for Camera{
    fn create_view(&self) -> Mat4 {
        let virt_view = Mat4::from_rotation_translation(                    
                    Quat::from_euler(
                        EulerRot::YXZ, 
                        self.rotation[0].to_radians(), 
                        self.rotation[1].to_radians(), 
                        self.rotation[2].to_radians(),
                    ), Vec3::from_array(self.position));
        virt_view
    }
}

pub trait CreateProjection {
    fn create_proj(&self) -> Mat4;
}

impl CreateProjection for Camera{
    fn create_proj(&self) -> Mat4 {
        let virt_proj: Mat4 = 
            if self.perspective{
                Mat4::perspective_rh(
                self.view_values[2].to_radians(),
                self.view_values[3], 
                self.view_values[0], 
                self.view_values[1],
            )
            }
            else {
                Mat4::orthographic_rh(
                    self.view_values[2] / -2.0, 
                    self.view_values[2] / 2.0, 
                    self.view_values[3] / -2.0, 
                    self.view_values[3] / 2.0, 
                    self.view_values[0], 
                    self.view_values[1],
                )
            };
        virt_proj
    }
}


pub trait GetUniformData{
    fn get_uniform_data(&self) -> Uniform_Camera;
}

impl GetUniformData for Camera{
    fn get_uniform_data(&self) -> Uniform_Camera {
        Uniform_Camera{
            view: self.create_view().to_cols_array_2d(),
            proj: self.create_proj().to_cols_array_2d(),
        }
    }
}