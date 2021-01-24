//! Bevy [`Bundle`] representing an SVG entity.

use crate::svg::Svg;
use bevy::{
    asset::Handle, ecs::Bundle, math::{Vec2, Vec3},
    render::{
        draw::{Draw, Visible}, mesh::Mesh, pipeline::{RenderPipeline, RenderPipelines},
        render_graph::base::MainPass,
    },
    sprite::{ColorMaterial, Sprite, QUAD_HANDLE, SPRITE_PIPELINE_HANDLE},
    transform::components::{GlobalTransform, Transform}
};


/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct SvgBundle {
    pub svg: Svg,
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl SvgBundle {
    /// Create a new [`SvgBundle`] from a [`Svg`].
    pub fn new(svg: Svg) -> SvgBundle {
        Self {
            svg,
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SPRITE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            main_pass: MainPass,
            draw: Default::default(),
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                ..Default::default()
            },
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }

    /// Specifies the 3D position at which the [`SvgBundle`] will be spawned.
    pub fn at_position(mut self, translation: Vec3) -> SvgBundle {
        self.transform = Transform::from_translation(translation);
        self
    }
}
