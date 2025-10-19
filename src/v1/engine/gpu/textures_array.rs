use wgpu::util::DeviceExt;
use crate::v1::engine::document::Document;
use crate::v1::engine::gpu::layer_data::{layer_to_data, DocumentData, LayerData};
use crate::v1::engine::image::buffer::ImageBuffer;
use crate::v1::engine::image::pixels::rgba::RGBAPixel;
use crate::v1::engine::layer::Layer;
use crate::v1::engine::painters::solid_painter::SolidPainter;

pub fn create_texture_array_from_images(
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    base_layer: &Layer<RGBAPixel<u8>>,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let layer_count = base_layer.get_layer_count() as u32;
    let width = base_layer.width as u32;
    let height = base_layer.height as u32;


    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: layer_count,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("texture_array"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let mut layer_data : Vec<LayerData> = Vec::new();
    // Upload each image into its array layer
    for (i, layer) in base_layer.children.iter().enumerate() {
        write_layer_to_gpu(queue, &texture, &mut layer_data, i, layer);
    }


    let layers_storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Layer Storage Buffer"),
        contents: bytemuck::cast_slice(&layer_data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let layer_count_uniform = DocumentData {
        layer_count,
        _padding: [0; 3],
    };

    let layer_count_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Layer Count Uniform"),
        contents: bytemuck::cast_slice(&[layer_count_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });


    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
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
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: layers_storage.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: layer_count_buffer.as_entire_binding(),
                },

            ],
            label: Some("diffuse_bind_group"),
        }
    );
    (texture_bind_group_layout, diffuse_bind_group)
}

pub fn create_img_array_buffer() -> Layer<RGBAPixel<u8>> {
    let mut document : Document<RGBAPixel<u8>> = Document::new(500, 500);

    let red: RGBAPixel<u8> = RGBAPixel::new(255, 0, 0, 255);
    let blue: RGBAPixel<u8> = RGBAPixel::new(0, 0, 255, 80);
    let red_solid: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(red);
    let blue_solid: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(blue);
    let blue_solid2: SolidPainter<RGBAPixel<u8>> = SolidPainter::new(blue);

    let red_layer: Layer<RGBAPixel<u8>> = Layer::new("Red Layer", 500, 500, Some(Box::new(red_solid)));
    let blue_layer: Layer<RGBAPixel<u8>> = Layer::new("Blue Layer", 500, 500, Some(Box::new(blue_solid)));
    // blue_layer.transform.x = 30;
    // blue_layer.transform.y = 30;
    let mut blue_layer2: Layer<RGBAPixel<u8>> = Layer::new("Blue Layer", 30, 30, Some(Box::new(blue_solid2)));
    blue_layer2.transform.x = 40;
    blue_layer2.transform.y = 40;

    document.base_layer.children.push(red_layer);
    document.base_layer.children.push(blue_layer);

    document.base_layer
}

fn buffer_to_array(buffer: &ImageBuffer<RGBAPixel<u8>>) -> Vec<u8> {
    let mut raw_data = Vec::with_capacity(buffer.width * buffer.height * 4);
    for pixel in buffer.data.iter() {
        raw_data.push(pixel.r);
        raw_data.push(pixel.g);
        raw_data.push(pixel.b);
        raw_data.push(pixel.a);
    }
    raw_data
}

fn write_layer_to_gpu(queue: &wgpu::Queue, texture: &wgpu::Texture, layer_data: &mut Vec<LayerData>, layer_index: usize, layer: &Layer<RGBAPixel<u8>>) {
    for(i, child) in layer.children.iter().enumerate() {
        write_layer_to_gpu(queue, texture, layer_data, layer_index + i, child);
    }
    let origin = wgpu::TexelCopyTextureInfo {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d {
            x: 0,
            y: 0,
            z: layer_index as u32,
        },
        aspect: wgpu::TextureAspect::All,
    };

    let data = buffer_to_array(&layer.render());
    let bytes_per_row = 4 * layer.width; // RGBA8
    let layout = wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(bytes_per_row as u32),
        rows_per_image: Some(layer.height as u32),
    };

    let layer_size = wgpu::Extent3d {
        width: layer.width as u32,
        height: layer.height as u32,
        depth_or_array_layers: 1,
    };

    queue.write_texture(origin, &data, layout, layer_size);
    layer_data.push(layer_to_data(layer, 0))
}