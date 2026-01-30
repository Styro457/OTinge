use wgpu::util::DeviceExt;
use crate::engine::document::Document;
use crate::engine::layer::Layer;
use crate::gpu_render::canvas::layer_data::{DocumentData, LayerData};

pub struct DocumentGPUHandler {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub layer_storage_buffer: wgpu::Buffer,
    pub document_uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,

    // Internal state
    pub data: DocumentData
}

impl DocumentGPUHandler {

    pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("document_bind_group_layout"),
            entries: &[
                // 0: Texture Array
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
                // 1: Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // 2: Layer Data (Storage)
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
                // 3: Document Uniforms (e.g., layer count)
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
        })
    }

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        document: &Document,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        // TODO: Get an actual value
        let tile_count = 64;
        let layer_count = document.layer_count();
        let width = document.tile_manager.get_tile_size().x;
        let height = document.tile_manager.get_tile_size().y;

        // --- 1. Create Texture ---
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: tile_count,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture_tile_array"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: document.buffer_type.get_wgpu_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // --- 2. Create Buffers ---
        let mut layer_data: Vec<LayerData> = Vec::with_capacity(layer_count as usize);
        Self::upload_all_layers(queue, &texture, &mut layer_data, 0, &document.base_layer);

        let layer_storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Layer Storage Buffer"),
            contents: bytemuck::cast_slice(&layer_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let document_data = DocumentData {
            layer_count,
            _padding: [0; 3],
        };
        let document_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Layer Count Uniform"),
            contents: bytemuck::cast_slice(&[document_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // --- 3. Create Sampler ---
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("document_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        // --- 4. Create Bind Group ---
        let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("document_bind_group"),
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: layer_storage_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: document_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            texture,
            sampler,
            layer_storage_buffer,
            document_uniform_buffer,
            bind_group,
            data: document_data,
        }
    }

    fn upload_all_layers(
        queue: &wgpu::Queue,
        texture: &wgpu::Texture,
        layer_data: &mut Vec<LayerData>,
        mut layer_index: usize,
        layer: &Layer,
    ) -> usize {
        // Render this layer (if it's not just a group)
        if layer.painter.is_some() {
            let tilegrid = layer.painter.unwrap().get_tilegrid();
            for tile in tilegrid.tiles.iter() {

            }

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer_index as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(rendered_buffer.bytes_per_pixel() * width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1, // We are writing one layer
                },
            );

            // Add this layer's metadata to the storage buffer
            layer_data_vec.push(layer_to_data(layer));
            layer_index += 1;
        }

        // Recurse into children
        for child in &layer.children {
            layer_index = Self::upload_all_layers(queue, texture, layer_data_vec, layer_index, child);
        }

        layer_index
    }

    /// Updates GPU buffers when the document changes.
    pub fn update(&mut self, queue: &wgpu::Queue, document: &Document) {

        // --- 1. Update Layer Textures & Metadata ---
        let mut layer_data: Vec<LayerData> = Vec::with_capacity(self.layer_count as usize);
        Self::upload_all_layers(queue, &self.texture, &mut layer_data, 0, &document.base_layer);

        // --- 2. Update Layer Storage Buffer ---
        queue.write_buffer(&self.layer_storage_buffer, 0, bytemuck::cast_slice(&layer_data));

        // --- 3. Update Uniforms ---
        let document_data = DocumentData {
            layer_count: self.layer_count,
            _padding: [0; 3],
        };
        queue.write_buffer(&self.document_uniform_buffer, 0, bytemuck::cast_slice(&[document_data]));
    }
}