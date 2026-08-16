use crate::image_handling::{image_to_texture, texture_size};
use wgpu::{BindGroup, BindGroupLayout, Extent3d, Texture};

use wgpu::{Device, SurfaceConfiguration, TextureView,};

pub fn generate_texture(
        device: &Device, 
        texture_name: &str, 
        file_type: &str, 
        surface_config: &SurfaceConfiguration,
    ) -> (
        Texture,
        Vec<u8>,
        Extent3d,
        BindGroup,
        BindGroupLayout,
        TextureView,
    ){
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
    let texture_image = image_to_texture(texture_name, file_type);
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
    (
        texture_asset,
        texture_image.0,
        texture_size,
        texture_bind_group,
        texture_bind_group_layout,
        depth_texture_view,
    )
}