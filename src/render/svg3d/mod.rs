use bevy::{asset::Handle, render::render_resource::Shader};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE:  Handle<Shader> = Handle::weak_from_u128(8514826640451853414);

pub use bundle::Svg3dBundle;
pub use plugin::RenderPlugin;
