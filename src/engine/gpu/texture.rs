use wgpu::{Device, Queue};
use crate::engine::document::Document;
use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::pixels::rgba::RGBAPixel;
use crate::engine::layer::Layer;
use crate::engine::painters::solid_painter::SolidPainter;

pub fn write_texture(queue: &mut Queue, device: &mut Device, image: ImageBuffer<RGBAPixel<u8>>) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let texture_size = wgpu::Extent3d {
        width: image.width as u32,
        height: image.height as u32,
        // All textures are stored as 3D, we represent our 2D texture
        // by setting depth to 1.
        depth_or_array_layers: 1,
    };

    let diffuse_texture = device.create_texture(
        &wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB, so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            // This is the same as with the SurfaceConfig. It
            // specifies what texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case) is
            // always supported. Note that using a different
            // texture format is not supported on the WebGL2
            // backend.
            view_formats: &[],
        }
    );

    // Convert the image to RGBA8 format
    let size = image.width * image.height;
    let mut diffuse_rgba = vec![0; size * 4];
    for i in 0..size {
        let pixel = &image.data[i];
        diffuse_rgba[i * 4] = pixel.r;
        diffuse_rgba[i * 4 + 1] = pixel.g;
        diffuse_rgba[i * 4 + 2] = pixel.b;
        diffuse_rgba[i * 4 + 3] = pixel.a;
    }

    queue.write_texture(
        // Tells wgpu where to copy the pixel data
        wgpu::TexelCopyTextureInfo {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        // The actual pixel data
        &diffuse_rgba,
        // The layout of the texture
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * image.width as u32),
            rows_per_image: Some(image.height as u32),
        },
        texture_size,
    );

    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

    let diffuse_bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                }
            ],
            label: Some("diffuse_bind_group"),
        }
    );
    (texture_bind_group_layout, diffuse_bind_group)

}

pub fn create_img_buffer() -> ImageBuffer<RGBAPixel<u8>> {
    let mut document : Document<RGBAPixel<u8>> = Document::new(500, 500);

    let red: RGBAPixel<u8> = RGBAPixel::new(255, 0, 0, 255);
    let blue: RGBAPixel<u8> = RGBAPixel::new(0, 0, 255, 80);
    let red_solid: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(red);
    let blue_solid: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(blue);
    let blue_solid2: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(blue);

    let red_layer: Layer<RGBAPixel<u8>> = Layer::new("Red Layer", 500, 500, Some(Box::new(red_solid)));
    let mut blue_layer: Layer<RGBAPixel<u8>> = Layer::new("Blue Layer", 50, 50, Some(Box::new(blue_solid)));
    blue_layer.transform.x = 30;
    blue_layer.transform.y = 30;
    let mut blue_layer2: Layer<RGBAPixel<u8>> = Layer::new("Blue Layer", 30, 30, Some(Box::new(blue_solid2)));
    blue_layer2.transform.x = 40;
    blue_layer2.transform.y = 40;

    document.base_layer.add_child(red_layer);
    document.base_layer.add_child(blue_layer);
    document.base_layer.add_child(blue_layer2);


    let image = document.render();
    image
}