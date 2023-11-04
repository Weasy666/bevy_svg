#import bevy_pbr::{
    mesh_types::Mesh,
    mesh_view_bindings,
    mesh_vertex_output::VertexOutput,
}

@group(2) @binding(0)
var<uniform> mesh: Mesh;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
