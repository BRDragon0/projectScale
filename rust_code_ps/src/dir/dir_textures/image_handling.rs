use image::{ImageReader};

use crate::dir::dir_fallbacks::fallback_defaults::FALLBACK_TEXTURE;

pub fn image_to_texture(texture_name: String,) -> (Vec<u8>, u32, u32){
    let img_buff = ImageReader::open(format!("assets/textures/{texture_name}"));
    let mut img = FALLBACK_TEXTURE.clone();
    if let Ok(value) = img_buff{
        img = value.decode().unwrap().to_rgba8();
    }
    else{
        println!("Error reading image file, file not found");
    }
    let img_x = img.width();
    let img_y = img.height();
    let img_data = img.into_raw();
    (img_data, img_x, img_y)
}

pub fn texture_size(img_width: u32, img_height: u32) -> wgpu::Extent3d{
    wgpu::Extent3d{
        width: img_width,
        height: img_height,
        depth_or_array_layers: 1
    }
}

pub fn texture_data<'a>(texture_asset: &'a wgpu::Texture, img_width: u32, img_height: u32) -> (wgpu::TexelCopyTextureInfo<'a>, wgpu::TexelCopyBufferLayout){
    (
    wgpu::TexelCopyTextureInfo{
        texture: &texture_asset,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    },
    wgpu::TexelCopyBufferLayout{
        offset: 0,
        bytes_per_row: Some(4 * img_width),
        rows_per_image: Some(img_height),
    },
    )
}