use std::collections::HashMap;

struct EffectRender {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    uniform_size: u64,
}

struct EffectRenderRegistry {
    effects: HashMap<String, EffectRender>,
}

