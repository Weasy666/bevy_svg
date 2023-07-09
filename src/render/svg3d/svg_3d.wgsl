#import bevy_pbr::mesh_types Mesh
#import bevy_pbr::mesh_view_bindings

@group(2) @binding(0)
var<uniform> mesh: Mesh;

#import bevy_pbr::mesh_vertex_output MeshVertexOutput

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
