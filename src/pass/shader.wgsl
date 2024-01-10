
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct Globals {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

struct Locals {
    m_matrix: mat4x4<f32>
}

@group(1) @binding(0)
var<uniform> locals: Locals;

@group(1) @binding(1)
var tex_view: texture_2d<f32>;
@group(1) @binding(2)
var tex_sampler: sampler;


@vertex
fn vs_main(in : VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = globals.view_proj  * locals.m_matrix * vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}



@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4f {
  return textureSample(tex_view, tex_sampler, in.tex_coords);
}
