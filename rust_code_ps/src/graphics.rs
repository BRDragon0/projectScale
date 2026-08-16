use std::borrow::Cow;
use std::default;
use std::collections::HashMap;
use bytemuck::{Pod, Zeroable};

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
use crate::objects_handling::{Object, ObjectStates};
use crate::image_handling::{image_to_texture, texture_data, texture_size};
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
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normals: [f32; 3],
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

    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: Some("vertex buffer"),
        size: (MAX_VERT * std::mem::size_of::<Vertex>()) as u64,
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



    //SECTION - TEXTURE STUFF
    //Create texture bind group
    let texture_bind_group_layout: BindGroupLayout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("texture_bind_group_layout"), 
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                         sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                         view_dimension: wgpu::TextureViewDimension::D2, 
                         multisampled: false 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry{
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    
    //Create nessisary data for texture write
    let texture_image = image_to_texture("cubiodTest", "png");
    let texture_size: wgpu::Extent3d = texture_size(texture_image.1, texture_image.2);
    let texture_asset = 
        device.create_texture(&wgpu::TextureDescriptor{
            label: Some("texture_asset"),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            view_formats: &[],
        });

    let texture_data = texture_data(&texture_asset, texture_image.1, texture_image.2);

    //Create sampler which choses which pixel from the image to select
    let sampler_asset =
        device.create_sampler(&wgpu::SamplerDescriptor{
            label: Some("sampler_asset"),
            //NOTE - Modes here can be changed
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

    let texture_view = texture_asset.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_texture = device.create_texture(&wgpu::TextureDescriptor{
        label: Some("depth_texture"),
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Depth32Float,
        size: wgpu::Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        view_formats: &[],
    });

    let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let texture_bind_group = 
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture_bind_group"), 
            layout: &texture_bind_group_layout, 
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler_asset),
                }
            ] 
        });

    queue.write_texture(
        texture_data.0,
        &texture_image.0,
        texture_data.1,
        texture_size
    );



    //Creates the pipeline layout with the uniform binding group
    //This is needed to add the uniform buffer to the pipeline
    let pipeline_layout: PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
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
        texture_bind_group,
        depth_texture_view,
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
                array_stride: std::mem::size_of::<Vertex>() as u64,
                attributes: &[
                    //Position attribute
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    //UV attribute
                    wgpu::VertexAttribute {
                        offset: 3 * 4,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                    //Normal attribute
                    wgpu::VertexAttribute {
                        offset: (3 * 4) + (2 * 4),
                        shader_location: 2,
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
        depth_stencil: Some(wgpu::DepthStencilState{
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
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
    texture_bind_group: wgpu::BindGroup,
    depth_texture_view: wgpu::TextureView,
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
            }})],

            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            });
            r_pass.set_pipeline(&self.render_pipeline);


            let x:&f32 = &(self.time.sin() * 30.0);

            //SECTION OBJECT TO VERTICES
            let mut verts: Object = Object::new("cubiod");
            let mut dupeMap: HashMap<[u16; 3], u16> = HashMap::new();
            verts.set_object_scale(Vec3::new(0.2, 0.2, 0.2));
            let data = verts.get_object_data();
            let mut faces = data.1;
            let mut vertices: Vec<Vertex> = Vec::new(); 
            let mut indices: Vec<u16> = Vec::new();
            for n in 0..faces.len(){
                if dupeMap.get(&faces[n]) != None{
                    indices.push(*dupeMap.get(&faces[n]).unwrap());
                }
                else{
                    dupeMap.insert(faces[n], vertices.len() as u16);
                    vertices.push(Vertex{ 
                            position: data.2[(faces[n][0] - 1) as usize], 
                            uv: data.3[(faces[n][1] - 1) as usize], 
                            normals: data.4[(faces[n][2] - 1) as usize],
                    });
                    indices.push((vertices.len() - 1) as u16);
                }
            }
            if indices.len() % 2 == 1 {
                indices.push(0);
            }
            
            for n in 0..vertices.len(){
                println!("v{:?}, vt,{:?}, vn{:?}", vertices[n].position, vertices[n].uv, vertices[n].normals)
            }

            for n in 0..(indices.len() / 3){
                println!("f {}, {}, {}", indices[(n * 3) + 0], indices[(n * 3) + 1], indices[(n * 3) + 2])
            }


            let update_camera =     
            camera_drg::Camera{
                position: [0.0, 0.0, -2.0],
                rotation: [self.time * 30.0, 0.0, 0.0],
                perspective: true,
                view_values: [0.1, 100.0, 90.0,
                self.window.inner_size().width as f32 / 
                self.window.inner_size().height as f32],
                // perspective: false,
                // view_values: [0.1, 10.0, 1.0, 1.0],
            };

            let update_uniform = update_camera.get_uniform_data();

            //ANCHOR - SET BUFFERS
            r_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            r_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            r_pass.set_bind_group(1, &self.texture_bind_group, &[]);
            r_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            self.queue.write_buffer(&self.uniform_buffer,0,bytemuck::bytes_of(&update_uniform));
            self.queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));

            let index_count = indices.len() as u32;
            r_pass.draw_indexed(0..index_count, 0, 0..1);
        } // `r_pass` dropped here
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}