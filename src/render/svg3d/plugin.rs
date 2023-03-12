use bevy::{
    app::{App, IntoSystemAppConfig, Plugin},
    core_pipeline::core_3d::Transparent3d,
    ecs::schedule::IntoSystemConfig,
    render::{
        ExtractSchedule,
        render_phase::AddRenderCommand, render_resource::SpecializedRenderPipelines, RenderSet,
    },
};

use crate::{render::svg3d::pipeline_3d, plugin::SvgSystem};

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom draw function and pipeline, and add our render systems
        app.init_resource::<pipeline_3d::Svg3dPipeline>()
            .init_resource::<SpecializedRenderPipelines<pipeline_3d::Svg3dPipeline>>()
            .add_render_command::<Transparent3d, pipeline_3d::DrawSvg3d>()
            .add_system(
                pipeline_3d::extract_svg_3d
                    .in_set(SvgSystem::ExtractSvgs)
                    .in_schedule(ExtractSchedule),
            )
            .add_system(
                pipeline_3d::queue_svg_3d.in_set(RenderSet::Queue)
            );
    }
}
