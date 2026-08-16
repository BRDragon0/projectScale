use image::{ImageReader};

pub fn image_to_texture(textureName: &str, fileType: &str) -> (Vec<u8>, u32, u32){
    let img = ImageReader::open(format!("../assets/textures/{textureName}.{fileType}")).unwrap().decode().unwrap().to_rgba8();
    let imgX = img.width();
    let imgY = img.height();
    let imgData = img.into_raw();
    (imgData, imgX, imgY)
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