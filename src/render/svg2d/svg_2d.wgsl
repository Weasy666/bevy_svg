#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_types
@group(1) @binding(0)
var<uniform> mesh: Mesh2d;


// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions

// The structure of the SVG vertex buffer as specified in our `SpecializedPipeline`.
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex.
    @builtin(position) clip_position: vec4<f32>,
    // Vertex color passed to the framgent shader in location(0).
    @location(0) color: vec4<f32>,
};

/// Entry point for the vertex shader.
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position into screen position
    out.clip_position = mesh2d_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    // Unpack the `u32` from the vertex buffer into the `vec4<f32>` used by the fragment shader
    out.color = vertex.blend_color;
    return out;
}

// The input of the fragment shader must correspond to the output of the vertex shader for all `location`s
struct FragmentInput {
    // The color is interpolated between vertices by default
    @location(0) blend_color: vec4<f32>,
};

/// Entry point for the fragment shader.
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.blend_color;
}
