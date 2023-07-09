#import bevy_sprite::mesh2d_types Mesh2d
#import bevy_sprite::mesh2d_view_bindings

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
