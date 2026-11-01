import bevy_sprite_render::mesh2d::{
    functions::{get_world_from_local, mesh2d_position_local_to_clip},
    view_bindings::view,
};
import bevy_svg::render::gradient::gradient_color;

@if(TONEMAP_IN_SHADER)
import bevy_core_pipeline::tonemapping;
@if(SRGB_OUTPUT)
import bevy_render::color_operations::linear_to_srgb;
@if(OKLAB_OUTPUT)
import bevy_render::color_operations::linear_rgb_to_oklab;

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
    out.clip_position = mesh2d_position_local_to_clip(
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
@if(TONEMAP_IN_SHADER)
    color = tonemapping::tone_mapping(color, view.color_grading);
@if(SRGB_OUTPUT)
    color = vec4(linear_to_srgb(color.rgb), color.a);
@if(OKLAB_OUTPUT)
    color = vec4(linear_rgb_to_oklab(color.rgb), color.a);
    return color;
}
