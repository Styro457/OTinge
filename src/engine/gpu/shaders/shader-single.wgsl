struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}


@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}


@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

//@fragment
//fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//var out: VertexOutput;
//    let texel_size = 1.0 / vec2(500, 500);
//    var color: vec4<f32> = vec4<f32>(0.0);
//
//    let radius = 4;
//    var count: f32 = 0.0;
//
//    for (var y: i32 = -radius; y <= radius; y = y + 1) {
//        for (var x: i32 = -radius; x <= radius; x = x + 1) {
//            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
//            color = color + textureSample(t_diffuse, s_diffuse, in.tex_coords + offset);
//            count = count + 1.0;
//        }
//    }
//    return color / count;
//}
