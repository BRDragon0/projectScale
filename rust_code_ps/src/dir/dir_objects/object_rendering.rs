use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Device, Buffer};
pub type Rc<T> = std::sync::Arc<T>;

use crate::dir::dir_objects::object_handling::{Object, ObjectStates};
use crate::dir::dir_uniform_buffers::state_buffer::create_state_buffer;

pub struct RenderObject{
    pub object: Object,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: u32,
    pub state_buffer: Buffer,
    pub state_bind_group: BindGroup,
    #[allow(dead_code)]
    pub state_bind_group_layout: BindGroupLayout,
    pub texture_bind_group: Rc<BindGroup>,
}

impl RenderObject {
    // The idiomatic constructor function
    pub fn new(device: &Device, texture_bind_group: Rc<BindGroup>, object: Object) -> Self {
        let (indices, vertices) = object.format_object();
        let (state_buffer, state_bind_group_layout, state_bind_group) = create_state_buffer(&device);
        Self {
            object,
            
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),

            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("index buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }),

            index_count: indices.len() as u32,

            state_buffer,
            state_bind_group,
            state_bind_group_layout,
            texture_bind_group,
        }
    }
}