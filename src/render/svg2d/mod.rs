use bevy::{asset::HandleUntyped, reflect::TypeUuid, render::render_resource::Shader};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8514826620251853414);

pub use bundle::Svg2dBundle;
pub use plugin::RenderPlugin;
