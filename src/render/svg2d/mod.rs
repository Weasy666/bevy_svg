use crate::{origin::Origin, svg::Svg};
use bevy::{
    asset::{Handle, uuid_handle},
    ecs::{component::Component, lifecycle::HookContext, world::DeferredWorld},
    mesh::Mesh2d,
    shader::Shader,
    sprite_render::MeshMaterial2d,
};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("00000000-0000-0000-762a-bdb29826d266");

pub use bundle::Svg2dBundle;
pub use plugin::RenderPlugin;

/// A component for 2D SVGs.
#[derive(Component, Default)]
#[require(Mesh2d, Origin)]
#[component(on_insert = svg_2d_on_insert)]
pub struct Svg2d(pub Handle<Svg>);

fn svg_2d_on_insert(mut world: DeferredWorld, ctx: HookContext) {
    let component = world.entity(ctx.entity).get_components::<&Svg2d>().unwrap();
    let handle = component.0.clone();
    let entity = world.entity(ctx.entity).id();
    let mut commands = world.commands();
    commands.entity(entity).insert(MeshMaterial2d(handle));
}
