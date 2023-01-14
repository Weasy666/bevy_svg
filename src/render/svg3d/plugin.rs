use bevy::{
    app::{App, Plugin},
    core_pipeline::core_3d::Transparent3d,
    render::{
        render_phase::AddRenderCommand, render_resource::SpecializedRenderPipelines, RenderStage,
    },
};

use crate::render::svg3d::pipeline_3d;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom draw function and pipeline, and add our render systems
        app.init_resource::<pipeline_3d::Svg3dPipeline>()
            .init_resource::<SpecializedRenderPipelines<pipeline_3d::Svg3dPipeline>>()
            .add_render_command::<Transparent3d, pipeline_3d::DrawSvg3d>()
            .add_system_to_stage(RenderStage::Extract, pipeline_3d::extract_svg_3d)
            .add_system_to_stage(RenderStage::Queue, pipeline_3d::queue_svg_3d);
    }
}
