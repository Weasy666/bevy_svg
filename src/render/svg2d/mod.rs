use bevy::{asset::Handle, render::render_resource::Shader};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(8_514_826_620_251_853_414);

pub use bundle::Svg2dBundle;
pub use plugin::RenderPlugin;
