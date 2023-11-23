
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> u_camera: mat4x4<f32>;

@vertex
fn vs_main(in : VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = u_camera * vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}



@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4f {
  return vec4<f32>(in.color, 1.0);
}
