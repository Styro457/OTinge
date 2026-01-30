// --- vertex ---
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// --- helper functions ---

fn rotate_vec(v: vec2<f32>, angle: f32) -> vec2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec2<f32>(
        v.x * c - v.y * s,
        v.x * s + v.y * c
    );
}

// --- painter struct (16B per painter) ---
struct LayerData {
    blend_mode: u32,
    opacity: f32,
    size: vec2<f32>,
    transform: mat4x4<f32>,
    //_pad: vec3<u32>,
};

// --- bindings ---
// @group(0)
//  binding 0: texture_2d_array<u16> -> layer data split in tiles
//  binding 1: texture_2d_array<f32> -> tiles
//  binding 2: sampler
//  binding 3: storage buffer (array<LayerData>)
//  binding 4: uniform { layer_count }

@group(0) @binding(0)
var layers: texture_2d_array<u32>;
@group(0) @binding(1)
var tiles: texture_2d_array<f32>;

@group(0) @binding(2)
var tex_sampler: sampler;

@group(0) @binding(3)
var<storage, read> layers_data: array<LayerData>;

@group(0) @binding(4)
var<uniform> u_layer_count: u32;

// --- blend helpers ---

fn over(base: vec4<f32>, over: vec4<f32>) -> vec4<f32> {
    let out_rgb = over.rgb * over.a + base.rgb * (1.0 - over.a);
    let out_a = over.a + base.a * (1.0 - over.a);
    return vec4<f32>(out_rgb, out_a);
}

fn blend_normal(b: vec4<f32>, o: vec4<f32>) -> vec4<f32> {
    return over(b, o);
}

fn blend_multiply(b: vec4<f32>, o: vec4<f32>) -> vec4<f32> {
    let blended = vec4<f32>(b.rgb * o.rgb, o.a);
    return over(b, blended);
}

fn blend_screen(b: vec4<f32>, o: vec4<f32>) -> vec4<f32> {
    let blended = vec4<f32>(1.0 - (1.0 - b.rgb) * (1.0 - o.rgb), o.a);
    return over(b, blended);
}

const EMPTY_TILE: u32 = 4294967295u;
const TILE_SIZE: f32 = 256.0;

// --- fragment shader ---

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var out_color = vec4<f32>(0, 0, 0, 0);

    let grid_size = vec2<f32>(textureDimensions(layers).xy);
    let canvas_size = grid_size * TILE_SIZE;

    let global_pixel_pos = vec4<f32>(in.tex_coords * canvas_size, 0.0, 1.0);

    for (var i: u32 = 0u; i < u_layer_count; i = i + 1u) {
        let layer = layers_data[i];

        let local_pos_vec = layer.transform * global_pixel_pos;
        let local_pos = local_pos_vec.xy;

        // Bounds Check
        if (local_pos.x < 0.0 || local_pos.y < 0.0 ||
            local_pos.x >= layer.size.x || local_pos.y >= layer.size.y) {
            continue;
        }

        let tile_indices = vec2<i32>(floor(local_pos / TILE_SIZE));
        let tile_id = textureLoad(layers, tile_indices, i, 0).r;

        if (tile_id == EMPTY_TILE) { continue; }

        let tile_uv = fract(local_pos / TILE_SIZE);
        let tex_color = textureSample(tiles, tex_sampler, tile_uv, tile_id);

        // --- Blend the layer ---
        // apply opacity
        let color = vec4<f32>(tex_color.rgb, tex_color.a * layer.opacity);

        // choose blend
        if (layer.blend_mode == 0u) {
            out_color = blend_normal(out_color, color);
        } else if (layer.blend_mode == 1u) {
            out_color = blend_multiply(out_color, color);
        } else if (layer.blend_mode == 2u) {
            out_color = blend_screen(out_color, color);
        } else {
            out_color = blend_normal(out_color, color);
        }
    }

    if (out_color.a > 0.0) {
        out_color = vec4<f32>(out_color.rgb / out_color.a, out_color.a);
    }

    return out_color;
}
