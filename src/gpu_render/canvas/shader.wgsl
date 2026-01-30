// ==== vertex ====
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

// ---- painter struct (16B per painter) ----
struct LayerData {
    blend_mode: u32,
    opacity: f32,
    texture_index: u32,
    _pad: u32,
};

// ==== bindings ====
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

// ==== blend helpers ====

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
const TILE_SIZE: f32 = 256;

// ==== fragment ====

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var out_color = vec4<f32>(0, 0, 0, 0);

    let layer_size = textureDimensions(layers);
    let grid_width = f32(layer_size.x);
    let grid_height = f32(layer_size.y);

    let tile_coord = vec2<u32>(
            u32(in.tex_coords.x * grid_width),
            u32(in.tex_coords.y * grid_height)
    );

    let tile_uv = vec2<f32>(
            fract(in.tex_coords.x * TILE_SIZE),
            fract(in.tex_coords.y * TILE_SIZE)
    );

    for (var i: u32 = 0u; i < u_layer_count; i = i + 1u) {
        let tile_index = textureLoad(layers, tile_coord, i, 0).r;

        if (tile_index == EMPTY_TILE) {
             continue;
        }

        let tex_color = textureSample(tiles, tex_sampler, tile_uv, tile_index);

        let layer_data = layers_data[i];

        // apply opacity
        let color = vec4<f32>(tex_color.rgb, tex_color.a * layer_data.opacity);

        // choose blend
        if (layer_data.blend_mode == 0u) {
            out_color = blend_normal(out_color, color);
        } else if (layer_data.blend_mode == 1u) {
            out_color = blend_multiply(out_color, color);
        } else if (layer_data.blend_mode == 2u) {
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
