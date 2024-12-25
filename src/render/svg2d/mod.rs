use crate::{origin::Origin, svg::Svg};
use bevy::{
    asset::Handle, ecs::{component::{self, Component, ComponentId}, world::DeferredWorld}, prelude::{Entity, Query}, render::{mesh::Mesh2d, render_resource::Shader}, sprite::MeshMaterial2d
};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(8_514_826_620_251_853_414);

pub use bundle::Svg2dBundle;
pub use plugin::RenderPlugin;

/// A component for 2D SVGs.
#[derive(Component, Default)]
#[require(Mesh2d, Origin)]
#[component(on_insert = svg_2d_on_insert)]
pub struct Svg2d(pub Handle<Svg>);

fn svg_2d_on_insert(mut world: DeferredWorld, entity: Entity, component_id: ComponentId) {
    let component = world.entity(entity).get_components::<&Svg2d>().unwrap();
    let handle = component.0.clone();
    let entity = world.entity(entity).id();
    let mut commands = world.commands();
    commands.entity(entity).insert(MeshMaterial2d(handle));
}
