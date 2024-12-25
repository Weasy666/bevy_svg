use crate::{origin::Origin, svg::Svg};
use bevy::{
    asset::Handle, ecs::component::Component, render::{mesh::Mesh2d, render_resource::Shader}, sprite::MeshMaterial2d,
};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(8_514_826_620_251_853_414);

pub use bundle::Svg2dBundle;
pub use plugin::RenderPlugin;

/// A component for 2D SVGs.
#[derive(Component, Default)]
#[require(Mesh2d, Origin, MeshMaterial2d<Svg>)]
pub struct Svg2d(pub Handle<Svg>);
