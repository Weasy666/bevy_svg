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

use bevy::{
    app::{App, Plugin},
    asset::{AssetEvent, Assets},
    ecs::{
        entity::Entity,
        message::MessageReader,
        query::{Added, Changed, Or},
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    log::debug,
    mesh::Mesh,
    prelude::{Last, PostUpdate},
};

#[cfg(feature = "2d")]
use bevy::mesh::Mesh2d;

#[cfg(feature = "3d")]
use bevy::mesh::Mesh3d;

use crate::{
    origin,
    render::{self, Svg2d, Svg3d},
    svg::Svg,
};

/// Set in which [`Svg`](crate::prelude::Svg2d)s get drawn.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct SvgSet;

/// A plugin that makes sure your [`Svg`]s get rendered
pub struct SvgRenderPlugin;

impl Plugin for SvgRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, origin::add_origin_state.in_set(SvgSet))
            .add_systems(Last, (origin::apply_origin, svg_mesh_linker.in_set(SvgSet)))
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
    Option<&'static Svg2d>,
    Option<&'static Svg3d>,
    Option<&'static mut Mesh2d>,
    Option<&'static mut Mesh3d>,
);

/// Bevy system which queries for all [`Svg`] bundles and adds the correct [`Mesh`] to them.
fn svg_mesh_linker(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    svgs: Res<Assets<Svg>>,
    mut query: Query<SvgMeshComponents>,
    changed_handles: Query<
        Entity,
        Or<(Changed<Svg2d>, Changed<Svg3d>, Added<Svg2d>, Added<Svg3d>)>,
    >,
) {
    for event in svg_events.read() {
        match event {
            AssetEvent::Added { .. } => (),
            AssetEvent::LoadedWithDependencies { id } => {
                for (.., mesh_2d, mesh_3d) in query.iter_mut().filter(|(_, svg_2d, svg_3d, ..)| {
                    svg_2d
                        .map(|x| x.0.id() == *id)
                        .or_else(|| svg_3d.map(|x| x.0.id() == *id))
                        .unwrap_or(false)
                }) {
                    let svg = svgs.get(*id).unwrap();
                    debug!(
                        "Svg `{}` created. Adding mesh component to entity.",
                        svg.name
                    );
                    #[cfg(feature = "2d")]
                    if let Some(mut mesh) = mesh_2d {
                        mesh.0 = svg.mesh.clone();
                    }
                    #[cfg(feature = "3d")]
                    if let Some(mut mesh) = mesh_3d {
                        mesh.0 = svg.mesh.clone();
                    }
                }
            }
            AssetEvent::Modified { id } => {
                for (.., mesh_2d, mesh_3d) in query.iter_mut().filter(|(_, svg_2d, svg_3d, ..)| {
                    svg_2d
                        .map(|x| x.0.id() == *id)
                        .or_else(|| svg_3d.map(|x| x.0.id() == *id))
                        .unwrap_or(false)
                }) {
                    let svg = svgs.get(*id).unwrap();
                    debug!(
                        "Svg `{}` modified. Changing mesh component of entity.",
                        svg.name
                    );
                    #[cfg(feature = "2d")]
                    if let Some(mut mesh) = mesh_2d.filter(|mesh| mesh.0 != svg.mesh) {
                        let old_mesh = mesh.0.clone();
                        mesh.0 = svg.mesh.clone();
                        meshes.remove(&old_mesh);
                    }
                    #[cfg(feature = "3d")]
                    if let Some(mut mesh) = mesh_3d.filter(|mesh| mesh.0 != svg.mesh) {
                        let old_mesh = mesh.clone();
                        mesh.0 = svg.mesh.clone();
                        meshes.remove(&old_mesh);
                    }
                }
            }
            AssetEvent::Removed { id } => {
                for (entity, ..) in query.iter_mut().filter(|(_, svg_2d, svg_3d, ..)| {
                    svg_2d
                        .map(|x| x.0.id() == *id)
                        .or_else(|| svg_3d.map(|x| x.0.id() == *id))
                        .unwrap_or(false)
                }) {
                    commands.entity(entity).despawn();
                }
            }
            AssetEvent::Unused { .. } => {
                // TODO: does anything need done here?
            }
        }
    }

    // Ensure all correct meshes are set for entities which have had modified handles
    for entity in changed_handles.iter() {
        let Ok((.., svg_2d, svg_3d, mesh_2d, mesh_3d)) = query.get_mut(entity) else {
            continue;
        };
        let Some(handle) = svg_2d.map_or_else(|| svg_3d.map(|x| &x.0), |x| Some(&x.0)) else {
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
        if let Some(mut mesh) = mesh_2d {
            mesh.0 = svg.mesh.clone();
        }
        #[cfg(feature = "3d")]
        if let Some(mut mesh) = mesh_3d {
            mesh.0 = svg.mesh.clone();
        }
    }
}
