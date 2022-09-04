use bevy::{
    asset::HandleUntyped,
    reflect::TypeUuid,
    render::render_resource::Shader,
};

mod bundle;
mod pipeline_3d;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8514826640451853414);

pub use bundle::Svg3dBundle;
pub use plugin::RenderPlugin;
