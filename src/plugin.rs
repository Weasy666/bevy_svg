//! Contains the plugin and its helper types.
//!
//! The [`Svg2dBundle`](crate::bundle::Svg2dBundle) provides a way to display an `SVG`-file
//! with minimal boilerplate.
//!
//! ## How it works
//! The user creates/loades a [`Svg2dBundle`](crate::bundle::Svg2dBundle) in a system.
//!
//! Then, in the [`Stage::SVG`](Stage::SVG), a mesh is created for each loaded [`Svg`] bundle.
//! Each mesh is then extracted in the [`RenderStage::Extract`](bevy::render::RenderStage) and added to the
//! [`RenderWorld`](bevy::render::RenderWorld).
//! Afterwards it is queued in the [`RenderStage::Queue`](bevy::render::RenderStage) for actual drawing/rendering.

#[cfg(feature = "3d")]
use std::ops::Deref;

use bevy::{
    app::{App, CoreSet, Plugin},
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Added, Changed, Or},
        schedule::{IntoSystemConfig, IntoSystemSetConfig, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    log::debug,
    render::mesh::Mesh,
};

#[cfg(feature = "2d")]
use bevy::sprite::Mesh2dHandle;

use crate::{origin, render, svg::Svg};

/// Stages for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum Stage {
    /// Stage in which [`Svg2dBundle`](crate::bundle::Svg2dBundle)s get drawn.
    SVG,
}

/// A plugin that makes sure your [`Svg`]s get rendered
pub struct SvgRenderPlugin;

impl Plugin for SvgRenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Stage::SVG.after(CoreSet::PostUpdate))
            .add_system(svg_mesh_linker.in_base_set(Stage::SVG))
            .add_system(origin::add_origin_state.in_base_set(Stage::SVG))
            .add_system(origin::apply_origin.in_base_set(CoreSet::PostUpdate))
            .add_plugin(render::SvgPlugin);
    }
}

#[cfg(feature = "2d")]
#[cfg(not(feature = "3d"))]
type SvgMeshComponents = (
    Entity,
    &'static Handle<Svg>,
    Option<&'static mut Mesh2dHandle>,
    Option<()>,
);
#[cfg(not(feature = "2d"))]
#[cfg(feature = "3d")]
type SvgMeshComponents = (
    Entity,
    &'static Handle<Svg>,
    Option<()>,
    Option<&'static mut Handle<Mesh>>,
);
#[cfg(all(feature = "2d", feature = "3d"))]
type SvgMeshComponents = (
    Entity,
    &'static Handle<Svg>,
    Option<&'static mut Mesh2dHandle>,
    Option<&'static mut Handle<Mesh>>,
);

/// Bevy system which queries for all [`Svg`] bundles and adds the correct [`Mesh`] to them.
fn svg_mesh_linker(
    mut commands: Commands,
    mut svg_events: EventReader<AssetEvent<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    svgs: Res<Assets<Svg>>,
    mut query: Query<SvgMeshComponents>,
    changed_handles: Query<Entity, Or<(Changed<Handle<Svg>>, Added<Handle<Svg>>)>>,
) {
    for event in svg_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                for (.., _mesh_2d, _mesh_3d) in
                    query.iter_mut().filter(|(_, svg, ..)| svg == &handle)
                {
                    let svg = svgs.get(handle).unwrap();
                    debug!(
                        "Svg `{}` created. Adding mesh component to entity.",
                        svg.name
                    );
                    #[cfg(feature = "2d")]
                    _mesh_2d.map(|mut mesh| mesh.0 = svg.mesh.clone());
                    #[cfg(feature = "3d")]
                    _mesh_3d.map(|mut mesh| *mesh = svg.mesh.clone());
                }
            }
            AssetEvent::Modified { handle } => {
                for (.., _mesh_2d, _mesh_3d) in
                    query.iter_mut().filter(|(_, svg, ..)| svg == &handle)
                {
                    let svg = svgs.get(handle).unwrap();
                    debug!(
                        "Svg `{}` modified. Changing mesh component of entity.",
                        svg.name
                    );
                    #[cfg(feature = "2d")]
                    _mesh_2d.filter(|mesh| mesh.0 != svg.mesh).map(|mut mesh| {
                        let old_mesh = mesh.0.clone();
                        mesh.0 = svg.mesh.clone();
                        meshes.remove(old_mesh);
                    });
                    #[cfg(feature = "3d")]
                    _mesh_3d
                        .filter(|mesh| mesh.deref() != &svg.mesh)
                        .map(|mut mesh| {
                            let old_mesh = mesh.clone();
                            *mesh = svg.mesh.clone();
                            meshes.remove(old_mesh);
                        });
                }
            }
            AssetEvent::Removed { handle } => {
                for (entity, ..) in query.iter_mut().filter(|(_, svg, ..)| svg == &handle) {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    // Ensure all correct meshes are set for entities which have had modified handles
    for entity in changed_handles.iter() {
        let Ok((.., handle, _mesh_2d, _mesh_3d))
            = query.get_mut(entity) else { continue };
        let Some(svg) = svgs.get(handle) else { continue };
        debug!(
            "Svg handle for entity `{:?}` modified. Changing mesh component of entity.",
            entity
        );
        #[cfg(feature = "2d")]
        _mesh_2d.map(|mut mesh| mesh.0 = svg.mesh.clone());
        #[cfg(feature = "3d")]
        _mesh_3d.map(|mut mesh| *mesh = svg.mesh.clone());
    }
}
