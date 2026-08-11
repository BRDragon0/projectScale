use std::borrow::Cow;
use std::default;

use glam::{Mat4, Quat, Vec3, Vec4, EulerRot};
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, PipelineLayout};
use wgpu::wgc::pipeline;
use wgpu::{
    Adapter, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FragmentState,
    Instance, Limits, LoadOp, MemoryHints, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, StoreOp, Surface,
    SurfaceConfiguration, TextureFormat, TextureViewDescriptor, VertexState,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

use crate::debug_tools_drg::debug_matrix_print;
use crate::{debug_tools_drg, math_drg};
use crate::camera_drg::{self, GetUniformData, Camera, Uniform_Camera};
use crate::obj_parser::{self, getObjectData};

//ANCHOR - SETTINGS
const MAX_VERT: usize = 65536;
const MAX_INDT: usize = 65536 * 3;

#[cfg(target_arch = "wasm32")]
pub type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Rc<T> = std::sync::Arc<T>;

#[repr(C)]
struct Vertex {
    position: [f32; 3],
}
//Create buffer states struct for getting the bunding group and the piepline
struct BuffferStates {
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
}

pub async fn create_graphics(window: Rc<Window>, proxy: EventLoopProxy<Graphics>) {

    let instance = Instance::default();
    let surface = instance.create_surface(Rc::clone(&window)).unwrap();
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(), // Power preference for the device
            force_fallback_adapter: false, // Indicates that only a fallback ("software") adapter can be used
            compatible_surface: Some(&surface), // Guarantee that the adapter can render to this surface
        })
        .await
        .expect("Could not get an adapter (GPU).");

    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(), // Specifies the required features by the device request. Fails if the adapter can't provide them.
            required_limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: MemoryHints::Performance,
            trace: Default::default(),
            experimental_features: Default::default(),
        })
        .await
        .expect("Failed to get device");

    // Get physical pixel dimensions inside the window
    let size = window.inner_size();
    // Make the dimensions at least size 1, otherwise wgpu would panic
    let width = size.width.max(1);
    let height = size.height.max(1);
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();

    surface.configure(&device, &surface_config);

    //TODO change f32 to vertex struct
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: Some("vertex buffer"),
        size: (MAX_VERT * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let index_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: Some("index buffer"),
        size: (MAX_INDT * std::mem::size_of::<u16>()) as u64,
        usage: wgpu::BufferUsages::INDEX| wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    //ANCHOR - UNIFORM BUFFER SETUP
    let default_camera = camera_drg::Camera{
        position: [0.0, 0.0, 0.0],
        rotation: [45.0, 0.0, 0.0],
        perspective: false,
        view_values: [0.0, 10.0, 2.0, 2.0],
    };

    let uniform_data = default_camera.get_uniform_data();

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("uniform buffer"),
        contents: bytemuck::bytes_of(&uniform_data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    //Create camera bind group layout
    //This creates the uniform bind group needed to add features like the camera position
    let uniform_bind_group_layout: BindGroupLayout = 
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
            label: Some("uniform_bind_group_layout"),
        }
    );

    //Creates uniform_bind_group
    //Connects to buffer
    let uniform_bind_group: BindGroup =
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),        
        }
    );

    //Creates the pipeline layout with the uniform binding group
    //This is needed to add the uniform buffer to the pipeline
    let pipeline_layout: PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
            label: Some("pipeline_layout"),
        }
    );

    let render_pipeline = create_pipeline(&device, surface_config.format, pipeline_layout);

    let gfx = Graphics {
        window: window.clone(),
        instance,
        surface,
        surface_config,
        adapter,
        device,
        queue,
        render_pipeline,
        vertex_buffer,
        uniform_bind_group,
        uniform_buffer,
        index_buffer,
        time: 0.00,
    };



    let _ = proxy.send_event(gfx);
}

fn create_pipeline(device: &Device, swap_chain_format: TextureFormat, layout: PipelineLayout,) -> RenderPipeline {

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            //Create a buffer to use shader.wgsl
            buffers: &[wgpu::VertexBufferLayout {
                step_mode: wgpu::VertexStepMode::Vertex,
                //6 array elements 4 bytes each
                array_stride: 6 * 4,
                attributes: &[
                    //Position atribute
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    //Colour atribute
                    wgpu::VertexAttribute {
                        offset: 3 * 4,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                ],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(swap_chain_format.into())],
            compilation_options: Default::default(),
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview: None,
        cache: None,
    })
}

#[derive(Debug)]
pub struct Graphics {
    window: Rc<Window>,
    instance: Instance,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    time: f32,
}

impl Graphics {
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self) {
        self.time += 0.016; // ~60fps
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture.");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            r_pass.set_pipeline(&self.render_pipeline);

            let x:&f32 = &(self.time.sin() * 30.0);

            let mut vertices: [f32; 48] = [
                 0.2,  0.2,  0.2, 1.0, 0.0, 0.0,
                -0.2,  0.2,  0.2, 0.0, 1.0, 0.0,
                 0.2, -0.2,  0.2, 0.0, 0.0, 1.0,
                -0.2, -0.2,  0.2, 1.0, 1.0, 1.0,
                 0.2,  0.2, -0.2, 1.0, 0.0, 0.0,
                -0.2,  0.2, -0.2, 0.0, 1.0, 0.0,
                 0.2, -0.2, -0.2, 0.0, 0.0, 1.0,
                -0.2, -0.2, -0.2, 1.0, 1.0, 1.0,
            ];

            let update_camera =     
            camera_drg::Camera{
                position: [0.0, 0.0, -0.4],
                rotation: [self.time * 10.0, 0.0, 0.0],
                perspective: true,
                view_values: [0.0, 100.0, 90.0,
                self.window.inner_size().width as f32 / 
                self.window.inner_size().height as f32],
                // perspective: false,
                // view_values: [0.1, 10.0, 1.0, 1.0],
            };
            
            //Faces
            let indices = [
                0 as u16, 2, 1, 
                1, 2, 3,
                0, 1, 5,
                0, 5, 4,
                0, 4, 2,
                2, 4, 6,
                4, 5, 7,
                4, 7, 6,
                1, 7, 5,
                1, 3, 7,
                2, 7, 3,
                2, 6, 7,
            ];

            let mut padded = bytemuck::bytes_of(&indices).to_vec();
            for i in 0..5{
                if padded.len() % 4 != 0{
                    padded.push(0);
                }
                else{
                    break;
                }
            }

            let update_uniform = update_camera.get_uniform_data();

            //ANCHOR - SET BUFFERS
            r_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            r_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            r_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::bytes_of(&vertices));
            self.queue.write_buffer(&self.uniform_buffer,0,bytemuck::bytes_of(&update_uniform));
            self.queue.write_buffer(&self.index_buffer, 0, &padded);

            let index_count = indices.len() as u32;
            r_pass.draw_indexed(0..index_count, 0, 0..1);
        } // `r_pass` dropped here
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}