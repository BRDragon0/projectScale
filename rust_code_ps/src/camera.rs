use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3, EulerRot};

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
pub struct CameraData {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub cpos: [f32; 4],
}

impl Camera{
    pub fn new() -> Camera{
        Camera { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], perspective: false, view_values: [0.1, 10.0, 1.0, 1.0] }
    }
}

impl CameraData{
    pub fn new() -> CameraData{
        Camera::new().get_camera_data()
    }
}



pub trait CreateView {
    fn create_view(&self) -> Mat4;
}

impl CreateView for Camera{
    fn create_view(&self) -> Mat4 {
        let virt_view = Mat4::from_rotation_translation(                    
                    Quat::from_euler(
                        EulerRot::XYZ, 
                        self.rotation[0].to_radians(), 
                        self.rotation[1].to_radians(), 
                        self.rotation[2].to_radians(),
                    ), Vec3::from_array(self.position));
        virt_view.inverse()
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


pub trait GetCameraData{
    fn get_camera_data(&self) -> CameraData;
}

impl GetCameraData for Camera{
    fn get_camera_data(&self) -> CameraData {
        CameraData{
            view: self.create_view().to_cols_array_2d(),
            proj: self.create_proj().to_cols_array_2d(),
            cpos: [self.position[0], self.position[1], self.position[2], 1.0],
        }
    }
}