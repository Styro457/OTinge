use wgpu::util::DeviceExt;
use crate::engine::document::Document;
use crate::engine::image::tiles::EMPTY_TILE;
use crate::engine::layer::Layer;
use crate::gpu_render::canvas::layer_data::{layer_to_data, DocumentData, LayerData};

pub struct DocumentGPUHandler {
    pub layers_texture: wgpu::Texture,
    pub tiles_texture: wgpu::Texture,
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
                // 0: Layers Array
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Uint,
                    },
                    count: None,
                },
                // 1: Tiles Array
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // 2: Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // 3: Layer Data (Storage)
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // 4: Document Uniforms (e.g., layer count)
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
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
        let layers_count = 128;
        let layer_count = document.layer_count();
        let tile_size = document.tile_manager.get_tile_size();
        let grid_size = document.tile_manager.get_grid_size(document.size);

        // --- 1. Create Textures ---

        let layers_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture_layer_array"),
            size:  wgpu::Extent3d {
                width: grid_size.x,
                height: grid_size.y,
                depth_or_array_layers: layers_count,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let tiles_texture_size = wgpu::Extent3d {
            width: tile_size.x,
            height: tile_size.y,
            depth_or_array_layers: tile_count,
        };

        let tiles_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture_tile_array"),
            size: tiles_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: document.tile_manager.buffer_type.get_wgpu_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // --- 2. Create Buffers ---
        let mut layer_data: Vec<LayerData> = Vec::with_capacity(layer_count as usize);
        Self::upload_all_layers(document, queue, &layers_texture, &tiles_texture, &mut layer_data, 0, &document.base_layer);

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
        let diffuse_layers_texture_view = layers_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_tiles_texture_view = tiles_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("document_bind_group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_layers_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&diffuse_tiles_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: layer_storage_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: document_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            layers_texture,
            tiles_texture,
            sampler,
            layer_storage_buffer,
            document_uniform_buffer,
            bind_group,
            data: document_data,
        }
    }

    // let mut layer_data: Vec<LayerData> = Vec::with_capacity(layer_count as usize);
    fn upload_all_layers(
        document: &Document,
        queue: &wgpu::Queue,
        layers_texture: &wgpu::Texture,
        tiles_texture: &wgpu::Texture,
        layer_data_vec: &mut Vec<LayerData>,
        mut layer_index: usize,
        layer: &Layer,
    ) -> usize {
        // Render this layer (if it's not just a group)
        if layer.painter.is_some() {
            let tile_grid = layer.painter.as_ref().unwrap().get_tilegrid();

            for tile_key in tile_grid.indirection.iter() {
                if *tile_key == EMPTY_TILE {
                    continue;
                }

                let tile = document.tile_manager.get_tile_by_id(*tile_key).unwrap();
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: tiles_texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: *tile_key,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    &tile.get_data(),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(tile.get_bytes_per_pixel() * document.tile_manager.get_tile_size().x),
                        rows_per_image: Some(document.tile_manager.get_tile_size().y),
                    },
                    wgpu::Extent3d {
                        width: document.tile_manager.get_tile_size().x,
                        height: document.tile_manager.get_tile_size().y,
                        depth_or_array_layers: 1, // We are writing one layer
                    },
                );
            }

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &layers_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer_index as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &tile_grid.get_data(),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * tile_grid.size.x),
                    rows_per_image: Some(tile_grid.size.y),
                },
                wgpu::Extent3d {
                    width: tile_grid.size.x,
                    height: tile_grid.size.y,
                    depth_or_array_layers: 1, // We are writing one layer
                },
            );

            // Add this layer's metadata to the storage buffer
            layer_data_vec.push(layer_to_data(layer));
            layer_index += 1;
        }

        // Recurse into children
        for child in &layer.children {
            layer_index = Self::upload_all_layers(document, queue, layers_texture, tiles_texture, layer_data_vec, layer_index, child);
        }

        layer_index
    }

    /// Updates GPU buffers when the document changes.
    pub fn update(&mut self, queue: &wgpu::Queue, document: &Document) {
        //
        // // --- 1. Update Layer Textures & Metadata ---
        // let mut layer_data: Vec<LayerData> = Vec::with_capacity(self.layer_count as usize);
        // Self::upload_all_layers(queue, &self.texture, &mut layer_data, 0, &document.base_layer);
        //
        // // --- 2. Update Layer Storage Buffer ---
        // queue.write_buffer(&self.layer_storage_buffer, 0, bytemuck::cast_slice(&layer_data));
        //
        // // --- 3. Update Uniforms ---
        // let document_data = DocumentData {
        //     layer_count: self.layer_count,
        //     _padding: [0; 3],
        // };
        // queue.write_buffer(&self.document_uniform_buffer, 0, bytemuck::cast_slice(&[document_data]));
    }
}