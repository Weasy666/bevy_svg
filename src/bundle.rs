//! Bevy [`Bundle`] representing an SVG entity.

use crate::{plugin::SVG_PIPELINE_HANDLE, svg::Svg, prelude::Origin};
use bevy::{
    asset::Handle, ecs::bundle::Bundle,
    render::{
        draw::{Draw, Visible}, mesh::Mesh, pipeline::{RenderPipeline, RenderPipelines},
        render_graph::base::MainPass,
    },
    transform::components::{GlobalTransform, Transform}
};


/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct SvgBundle {
    pub svg: Handle<Svg>,
    /// Origin of the coordinate system and as such the origin for the Bevy position.
    pub origin: Origin,
    pub mesh: Handle<Mesh>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for SvgBundle {
    /// Create a new [`SvgBundle`].
    fn default() -> Self {
        Self {
            svg: Default::default(),
            origin: Default::default(),
            mesh: Default::default(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SVG_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            main_pass: MainPass,
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
