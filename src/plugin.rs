//! Contains the plugin and its helper types.
//!
//! The [`Svg2dBundle`](crate::bundle::Svg2dBundle) provides a way to display an `SVG`-file
//! with minimal boilerplate.
//!
//! ## How it works
//! The user creates/loades a [`Svg2dBundle`](crate::bundle::Svg2dBundle) in a system.
//!
//! Then, in the [`Set::SVG`](Set::SVG), a mesh is created for each loaded [`Svg`] bundle.
//! Each mesh is then extracted in the [`RenderSet::Extract`](bevy::render::RenderSet) and added to the
//! [`RenderWorld`](bevy::render::RenderWorld).
//! Afterwards it is queued in the [`RenderSet::Queue`](bevy::render::RenderSet) for actual drawing/rendering.

#[cfg(feature = "3d")]
use std::ops::Deref;

use bevy::{
    app::{App, Plugin},
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Added, Changed, Or},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    log::debug,
    prelude::{Last, PostUpdate},
    render::mesh::Mesh,
};

#[cfg(feature = "2d")]
use bevy::sprite::Mesh2dHandle;

use crate::{origin, render, svg::Svg};

/// Sets for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Set {
    /// Set in which [`Svg2dBundle`](crate::bundle::Svg2dBundle)s get drawn.
    SVG,
}

/// A plugin that makes sure your [`Svg`]s get rendered
pub struct SvgRenderPlugin;

impl Plugin for SvgRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (origin::add_origin_state.in_set(Set::SVG),))
            .add_systems(
                Last,
                (
                    origin::apply_origin,
                    svg_mesh_linker.in_set(Set::SVG),
                ),
            )
            .add_plugins(render::SvgPlugin);
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
    for event in svg_events.read() {
        match event {
            AssetEvent::Added { .. } => (),
            AssetEvent::LoadedWithDependencies { id } => {
                for (.., _mesh_2d, _mesh_3d) in query
                    .iter_mut()
                    .filter(|(_, handle, ..)| handle.id() == *id)
                {
                    let svg = svgs.get(*id).unwrap();
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
            AssetEvent::Modified { id } => {
                for (.., _mesh_2d, _mesh_3d) in query
                    .iter_mut()
                    .filter(|(_, handle, ..)| handle.id() == *id)
                {
                    let svg = svgs.get(*id).unwrap();
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
            AssetEvent::Removed { id } | AssetEvent::Unused { id } => {
                for (entity, ..) in query.iter_mut().filter(|(_, svg, ..)| svg.id() == *id) {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    // Ensure all correct meshes are set for entities which have had modified handles
    for entity in changed_handles.iter() {
        let Ok((.., handle, _mesh_2d, _mesh_3d)) = query.get_mut(entity) else {
            continue;
        };
        let Some(svg) = svgs.get(handle) else {
            continue;
        };
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
