import bevy_pbr::render::mesh_functions::{get_world_from_local, mesh_position_local_to_clip};
import bevy_svg::render::gradient::gradient_color;

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var gradients: texture_2d<f32>;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) gradient_position: vec2<f32>,
    @location(3) gradient_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) gradient_position: vec2<f32>,
    @location(2) @interpolate(flat) gradient_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.color = vertex.color;
    out.gradient_position = vertex.gradient_position;
    out.gradient_index = vertex.gradient_index;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;
    if in.gradient_index != 0u {
        color *= gradient_color(gradients, in.gradient_index - 1u, in.gradient_position);
    }
    return color;
}
