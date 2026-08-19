use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Device, Buffer};

use crate::camera::{CameraData};

pub fn create_camera_buffer(device: &Device) -> (Buffer, BindGroup, BindGroupLayout){
    let def_cam = CameraData::new();
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("camera buffer"),
        contents: bytemuck::bytes_of(&def_cam),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let camera_bind_group_layout: BindGroupLayout = 
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("camera_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer{
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        }
    );
    let camera_bind_group: BindGroup =
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),        
        }
    );
    (camera_buffer, camera_bind_group, camera_bind_group_layout)
}