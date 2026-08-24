use std::borrow::Cow;
use std::default;
use std::collections::HashMap;
use bytemuck::{Pod, Zeroable, bytes_of};

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

use crate::dir::dir_objects::object_handling::{Object, ObjectStates};
use crate::dir::dir_objects::object_rendering::RenderObject;
use crate::dir::dir_textures::image_handling::texture_data;
use crate::dir::dir_uniform_buffers::state_buffer::create_state_buffer;
use crate::dir::dir_textures::texture_control::{create_texture_bind_group_layout, generate_depth_texture, generate_texture};
use crate::dir::dir_uniform_buffers::camera_buffer::create_camera_buffer;
use crate::camera::{GetCameraData, Camera};

#[cfg(target_arch = "wasm32")]
pub type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Rc<T> = std::sync::Arc<T>;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normals: [f32; 3],
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

    //SECTION - Camera setup
    let (camera_buffer, camera_bind_group, camera_bind_group_layout) = create_camera_buffer(&device);
    let camera_state =     
            Camera{
                position: [0.0, 0.0, -1.0],
                rotation: [180.0, 0.0, 0.0],
                perspective: true,
                near: 0.1,
                far: 10.0,
                width: width as f32,
                height: height as f32,
                fov: 90.0,
            };

    //SECTION - Texure controls
    let (depth_texture, depth_texture_view) = generate_depth_texture(&device, &surface_config);

    //SECTION OBJECTS
    let mut obj1: Object = Object::new("toriod");
    obj1.set_object_scale(Vec3::new(0.2, 0.2, 0.2));
    obj1.texture_file = "cubiodTest.png".to_string();
    let mut obj2: Object = Object::new("cubiod");
    obj2.set_object_scale(Vec3::new(0.2, 0.2, 0.2));
    obj2.set_object_position(Vec3 { x: 1.0, y: 0.0, z: 0.0 });

    let objects_vec: Vec<Object> = vec![obj1, obj2];
    let mut texture_map: HashMap<String, Rc<BindGroup>> = HashMap::new();
    let mut render_objects: Vec<RenderObject> = Vec::new();

    let texture_bind_group_layout = create_texture_bind_group_layout(&device);

    for c_object in objects_vec{
        if texture_map.get(&c_object.texture_file) != None{
            render_objects.push(
                RenderObject::new(
                    &device, 
                    texture_map.get(&c_object.texture_file).unwrap().clone(),
                    c_object,
                ));
        }
        else{
            let (texture_asset, texture_raw, texture_size,texture_bind_group)
             = generate_texture(&device, c_object.texture_file.clone(), &texture_bind_group_layout);
            let (texture_info, texture_layout)
             = texture_data(&texture_asset, texture_size.width, texture_size.height);
            queue.write_texture(texture_info, &texture_raw, texture_layout, texture_size);

            texture_map.insert(c_object.texture_file.clone(), Rc::new(texture_bind_group));
            render_objects.push(
                RenderObject::new(
                    &device, 
                    texture_map.get(&c_object.texture_file).unwrap().clone(),
                    c_object, 
            ));
        }
    }

    //Matrix buffer setup
    let matrix_bind_group_layout = create_state_buffer(&device).1;

    //Creates the pipeline layout with the uniform binding group
    //This is needed to add the uniform buffer to the pipeline
    let pipeline_layout: PipelineLayout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout, &matrix_bind_group_layout],
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
        camera_state,
        camera_buffer,
        camera_bind_group,
        render_objects,
        depth_texture,
        depth_texture_view,
        last_frame: std::time::Instant::now(),
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

//#[derive(Debug)]
pub struct Graphics {
    window: Rc<Window>,
    instance: Instance,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    camera_state: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    render_objects: Vec<RenderObject>,
    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,
    last_frame: std::time::Instant,
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
        (self.depth_texture, self.depth_texture_view) = generate_depth_texture(&self.device, &self.surface_config);
        self.camera_state.width = self.window.inner_size().width as f32;
        self.camera_state.height = self.window.inner_size().height as f32;
    }

    pub fn update(&mut self){
        let dt =  std::time::Instant::now().duration_since(self.last_frame);
        self.last_frame = std::time::Instant::now();
        self.time += dt.as_secs_f32();

        //SECTION - EDIT OBJECTS HERE
        self.render_objects[0].object.set_object_rotation(Vec3 { x: self.time * 30.0, y: self.time * 30.0, z: self.time * 30.0 });
        self.render_objects[1].object.set_object_position(Vec3 { x: (self.time * 30.0).to_radians().sin(), y: (self.time * 30.0).to_radians().cos(), z: 0.0});

        //SECTION - EDIT CAMERA HERE
    }

    pub fn draw(&mut self) {
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
                        load: LoadOp::Clear(Color {r: 0.2, g: 0.2, b: 0.2, a: 1.0}),
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

            //ANCHOR - SET BUFFERS
            r_pass.set_pipeline(&self.render_pipeline);
            r_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            self.queue.write_buffer(&self.camera_buffer,0,bytemuck::bytes_of(&self.camera_state.get_camera_data()));

            //write to queue per object
            for render_object in &mut self.render_objects{
                self.queue.write_buffer(&render_object.state_buffer, 0,
                    bytes_of(&render_object.object.get_object_state().to_cols_array_2d())
                );
                r_pass.set_vertex_buffer(0, render_object.vertex_buffer.slice(..));
                r_pass.set_index_buffer(render_object.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                r_pass.set_bind_group(1, render_object.texture_bind_group.as_ref(), &[]);
                r_pass.set_bind_group(2, &render_object.state_bind_group, &[]);
                r_pass.draw_indexed(0..render_object.index_count, 0, 0..1);
            }
        } // `r_pass` dropped here
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}