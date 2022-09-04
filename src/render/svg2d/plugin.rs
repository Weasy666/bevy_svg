use bevy::{
    app::{App, Plugin},
    core_pipeline::core_2d::Transparent2d,
    render::{
        render_phase::AddRenderCommand,
        render_resource::SpecializedRenderPipelines,
        RenderStage,
    },
};

use crate::render::svg2d::pipeline_2d;


/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;


impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom draw function and pipeline, and add our render systems
        app
            .init_resource::<pipeline_2d::Svg2dPipeline>()
            .init_resource::<SpecializedRenderPipelines<pipeline_2d::Svg2dPipeline>>()
            .init_resource::<pipeline_2d::ExtractedSvgs2d>()
            .add_render_command::<Transparent2d, pipeline_2d::DrawSvg2d>()
            .add_system_to_stage(RenderStage::Extract, pipeline_2d::extract_svg_2d)
            .add_system_to_stage(RenderStage::Prepare, pipeline_2d::prepare_svg_2d)
            .add_system_to_stage(RenderStage::Queue, pipeline_2d::queue_svg_2d);
    }
}
