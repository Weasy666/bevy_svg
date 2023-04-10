#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(2) @binding(0)
var<uniform> mesh: Mesh;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}
