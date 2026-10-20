import bevy_sprite_render::mesh2d::{
    types::Mesh2d,
    view_bindings::view,
    vertex_output::VertexOutput,
};

@if(TONEMAP_IN_SHADER)
import bevy_core_pipeline::tonemapping;


@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
@if(VERTEX_COLORS) {
    var color = in.color;
@if(TONEMAP_IN_SHADER)
    color = tonemapping::tone_mapping(color, view.color_grading);
    return color;
}
@else {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}
}
