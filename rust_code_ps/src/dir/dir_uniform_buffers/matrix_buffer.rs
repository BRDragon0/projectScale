use glam::Mat4;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout};
use wgpu::{Device, Buffer};

pub fn create_matrix_buffer(device: &Device, object_transform: Mat4) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup){
    let buffer: Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("uniform buffer"),
        contents: bytemuck::bytes_of(&object_transform.to_cols_array_2d()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout: BindGroupLayout = 
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
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
            label: Some("bind_group_layout"),
        }
    );

    let bind_group: BindGroup =
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),        
        }
    );

    (buffer, bind_group_layout, bind_group)
}