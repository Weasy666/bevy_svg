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

use std::ops::Deref;

use bevy::{
    app::{App, Plugin},
    asset::{AddAsset, AssetEvent, Assets, Handle},
    ecs::{
        entity::Entity,
        event::EventReader,
        schedule::{StageLabel, SystemStage},
        system::{Commands, Query, Res, ResMut}
    },
    hierarchy::DespawnRecursiveExt,
    log::debug,
    render::mesh::Mesh,
    sprite::Mesh2dHandle,
};
use lyon_tessellation::{FillTessellator, StrokeTessellator};

use crate::{loader::SvgAssetLoader, render, svg::Svg};


/// Stages for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum Stage {
    /// Stage in which [`Svg2dBundle`](crate::bundle::Svg2dBundle)s get drawn.
    SVG,
}

/// A plugin that provides resources and a system to draw [`Svg`]s.
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app
            .add_asset::<Svg>()
            .init_asset_loader::<SvgAssetLoader>()
            .insert_resource(fill_tess)
            .insert_resource(stroke_tess)
            .add_stage_after(
                bevy::app::CoreStage::Update,
                Stage::SVG,
                SystemStage::parallel(),
            )
            .add_system_to_stage(Stage::SVG, svg_mesh_linker)
            .add_plugin(render::SvgPlugin);
    }
}

/// Bevy system which queries for all [`Svg`] bundles and adds the correct [`Mesh`] to them.
fn svg_mesh_linker(
    mut commands: Commands,
    mut svg_events: EventReader<AssetEvent<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    svgs: Res<Assets<Svg>>,
    mut query: Query<
        (Entity, &Handle<Svg>, Option<&mut Mesh2dHandle>, Option<&mut Handle<Mesh>>),
    >,
) {
    for event in svg_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                for (.., mesh_2d, mesh_3d) in query.iter_mut().filter(|(_, svg, ..)| svg == &handle) {
                    let svg = svgs.get(handle).unwrap();
                    debug!("Svg `{}` created. Adding mesh component to entity.", svg.name);
                    mesh_2d.map(|mut mesh| mesh.0 = svg.mesh.clone());
                    mesh_3d.map(|mut mesh| *mesh = svg.mesh.clone());
                }
            },
            AssetEvent::Modified { handle } => {
                for (.., mesh_2d, mesh_3d) in query.iter_mut().filter(|(_, svg, ..)| svg == &handle) {
                    let svg = svgs.get(handle).unwrap();
                    debug!("Svg `{}` modified. Changing mesh component of entity.", svg.name);
                    mesh_2d.filter(|mesh| mesh.0 != svg.mesh)
                        .map(|mut mesh| {
                            let old_mesh = mesh.0.clone();
                            mesh.0 = svg.mesh.clone();
                            meshes.remove(old_mesh);
                        });
                    mesh_3d.filter(|mesh| mesh.deref() != &svg.mesh)
                        .map(|mut mesh| {
                            let old_mesh = mesh.clone();
                            *mesh = svg.mesh.clone();
                            meshes.remove(old_mesh);
                        });
                }
            },
            AssetEvent::Removed { handle } => {
                for (entity, ..) in query.iter_mut().filter(|(_, svg, ..)| svg == &handle) {
                    commands.entity(entity).despawn_recursive();
                }
            },
        }
    }
}
