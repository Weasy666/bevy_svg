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

use bevy::{
    app::{App, Plugin},
    asset::{AddAsset, AssetEvent, Assets, Handle},
    ecs::{
        entity::Entity,
        event::EventReader,
        schedule::{StageLabel, SystemStage},
        system::{Query, Res, ResMut}
    },
    log::trace,
    math::Vec3,
    render::mesh::Mesh,
    sprite::Mesh2dHandle,
    transform::components::Transform,
};
use lyon_tessellation::{FillTessellator, StrokeTessellator};

use crate::{Convert, loader::SvgAssetLoader, render::tessellation, svg::{Origin, Svg}};


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
            .add_system_to_stage(Stage::SVG, svg_mesh_maker)
            .add_plugin(crate::render::SvgPlugin);
    }
}

/// Bevy system which queries for all [`Svg`](crate::svg::Svg)s and tessellates them into a mesh.
fn svg_mesh_maker(
    mut svg_events: EventReader<AssetEvent<Svg>>,
    svgs: Res<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (Entity, &Handle<Svg>, Option<&mut Mesh2dHandle>, Option<&mut Handle<Mesh>>, &Origin, &mut Transform),
    >,
) {
    for event in svg_events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                let mut tesselated_mesh = None;
                for (_, _, mesh_2d, mesh_3d, origin, mut transform) in query.iter_mut().filter(|(_, svg, _, _, _, _)| svg == &handle) {
                    let svg = svgs.get(handle).unwrap();
                    if tesselated_mesh.is_none() {
                        trace!("Make mesh for SVG: {}", svg.name);
                        let buffer = tessellation::generate_buffer(&svg, &mut fill_tess, &mut stroke_tess);
                        tesselated_mesh = Some(meshes.add(buffer.convert()));
                    } else {
                        trace!("Mesh for SVG `{}` already available, copying handle", svg.name);
                    }

                    let translation = match origin {
                        Origin::Center => transform.translation + Vec3::new(
                            -svg.width as f32 * transform.scale.x / 2.0,
                            svg.height as f32 * transform.scale.y / 2.0,
                            0.0
                        ),
                        Origin::TopLeft => transform.translation,
                    };
                    transform.translation = translation;

                    let new_mesh = tesselated_mesh.as_ref().unwrap().clone();
                    if let Some(mut mesh_2d) = mesh_2d {
                        mesh_2d.0 = new_mesh.clone();
                    }
                    if let Some(mut mesh_3d) = mesh_3d {
                        *mesh_3d = new_mesh;
                    }
                }
            },
            AssetEvent::Removed { handle } => {
                let _bundle = query.iter_mut().filter(|(_, svg, _, _, _, _)| svg == &handle).next();
                //TODO:
            },
        }
    }
}
