use bevy::{
    asset::Handle,
    ecs::component::Component,
    render::{mesh::Mesh3d, render_resource::Shader},
};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(8_514_826_640_451_853_414);

pub use bundle::Svg3dBundle;
pub use plugin::RenderPlugin;

use crate::{origin::Origin, svg::Svg};

#[derive(Component, Default)]
#[require(Mesh3d, Origin)]
pub struct Svg3d(pub Handle<Svg>);
