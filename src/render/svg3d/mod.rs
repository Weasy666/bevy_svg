use bevy::{
    asset::Handle,
    ecs::{
        component::{Component, ComponentId},
        world::DeferredWorld,
    },
    pbr::MeshMaterial3d,
    prelude::Entity,
    render::{mesh::Mesh3d, render_resource::Shader},
};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(8_514_826_640_451_853_414);

pub use bundle::Svg3dBundle;
pub use plugin::RenderPlugin;

use crate::{origin::Origin, svg::Svg};

/// A component for 3D SVGs.
#[derive(Component, Default)]
#[require(Mesh3d, Origin, MeshMaterial3d<Svg>)]
#[component(on_insert = svg_3d_on_insert)]
pub struct Svg3d(pub Handle<Svg>);

fn svg_3d_on_insert(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    let component = world.entity(entity).get_components::<&Svg3d>().unwrap();
    let handle = component.0.clone();
    let entity = world.entity(entity).id();
    let mut commands = world.commands();
    commands.entity(entity).insert(MeshMaterial3d(handle));
}
