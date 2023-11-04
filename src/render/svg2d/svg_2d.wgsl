#import bevy_sprite::{
    mesh2d_types::Mesh2d,
    mesh2d_view_bindings,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
