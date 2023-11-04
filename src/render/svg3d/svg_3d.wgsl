#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput}
    mesh_types::Mesh,
    mesh_view_bindings,
}

@group(2) @binding(0)
var<uniform> mesh: Mesh;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var out: FragmentOutput;
    out.color = in.color;
    return out;
}
