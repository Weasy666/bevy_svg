use bevy::{
    asset::{Handle, uuid_handle},
    ecs::{component::Component, lifecycle::HookContext, world::DeferredWorld},
    mesh::Mesh3d,
    pbr::MeshMaterial3d,
    shader::Shader,
};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("00000000-0000-0000-762a-bdb74c2a5c66");

pub use bundle::Svg3dBundle;
pub use plugin::RenderPlugin;

use crate::{origin::Origin, svg::Svg};

/// A component for 3D SVGs.
#[derive(Component, Default)]
#[require(Mesh3d, Origin, MeshMaterial3d<Svg>)]
#[component(on_insert = svg_3d_on_insert)]
pub struct Svg3d(pub Handle<Svg>);

fn svg_3d_on_insert(mut world: DeferredWorld, ctx: HookContext) {
    let component = world.entity(ctx.entity).get_components::<&Svg3d>().unwrap();
    let handle = component.0.clone();
    let entity = world.entity(ctx.entity).id();
    let mut commands = world.commands();
    commands.entity(entity).insert(MeshMaterial3d(handle));
}
