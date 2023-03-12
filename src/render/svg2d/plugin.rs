use bevy::{
    app::{App, IntoSystemAppConfig, Plugin},
    core_pipeline::core_2d::Transparent2d,
    ecs::schedule::IntoSystemConfig,
    render::{
        ExtractSchedule,
        render_phase::AddRenderCommand, render_resource::SpecializedRenderPipelines, RenderSet,
    },
};

use crate::{render::svg2d::pipeline_2d, plugin::SvgSystem};

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom draw function and pipeline, and add our render systems
        app.init_resource::<pipeline_2d::Svg2dPipeline>()
            .init_resource::<SpecializedRenderPipelines<pipeline_2d::Svg2dPipeline>>()
            .add_render_command::<Transparent2d, pipeline_2d::DrawSvg2d>()
            .add_system(
                pipeline_2d::extract_svg_2d
                    .in_set(SvgSystem::ExtractSvgs)
                    .in_schedule(ExtractSchedule),
            )
            .add_system(
                pipeline_2d::queue_svg_2d.in_set(RenderSet::Queue)
            );
    }
}
