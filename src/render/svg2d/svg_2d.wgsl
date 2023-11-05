#import bevy_sprite::{
    mesh2d_types::Mesh2d,
    mesh2d_view_bindings::view,
    mesh2d_vertex_output::VertexOutput,
}

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif


@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
#ifdef VERTEX_COLORS
    var color = in.color;
#ifdef TONEMAP_IN_SHADER
    color = tonemapping::tone_mapping(color, view.color_grading);
#endif
    return color;
#else
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
#endif
}
