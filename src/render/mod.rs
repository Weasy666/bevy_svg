use bevy::{
    app::{App, Plugin},
    asset::{Assets, HandleUntyped},
    core_pipeline::{Transparent2d, Transparent3d},
    reflect::TypeUuid,
    render::{
        render_phase::AddRenderCommand,
        render_resource::{Shader, SpecializedPipelines},
        RenderApp, RenderStage,
    },
};
use lyon_tessellation::{FillTessellator, StrokeTessellator};

mod pipeline_2d;
mod pipeline_3d;
pub(crate) mod tessellation;
mod vertex_buffer;


/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct SvgPlugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8514826620251853414);
pub const SVG_3D_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8514826640451853414);

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        // Load SVG shader
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            SVG_2D_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("svg_2d.wgsl")),
        );
        shaders.set_untracked(
            SVG_3D_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("svg_3d.wgsl")),
        );
        app
            .insert_resource(fill_tess)
            .insert_resource(stroke_tess);
        // Register our custom draw function and pipeline, and add our render systems
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_render_command::<Transparent2d, pipeline_2d::DrawSvg2d>()
            .init_resource::<pipeline_2d::Svg2dPipeline>()
            .init_resource::<SpecializedPipelines<pipeline_2d::Svg2dPipeline>>()
            .init_resource::<pipeline_2d::ExtractedSvgs2d>()
            .add_system_to_stage(RenderStage::Extract, pipeline_2d::extract_svg_2d)
            .add_system_to_stage(RenderStage::Queue, pipeline_2d::queue_svg_2d);
        render_app
            .add_render_command::<Transparent3d, pipeline_3d::DrawSvg3d>()
            .init_resource::<pipeline_3d::Svg3dPipeline>()
            .init_resource::<SpecializedPipelines<pipeline_3d::Svg3dPipeline>>()
            .init_resource::<pipeline_3d::ExtractedSvgs3d>()
            .add_system_to_stage(RenderStage::Extract, pipeline_3d::extract_svg_3d)
            .add_system_to_stage(RenderStage::Queue, pipeline_3d::queue_svg_3d);
    }
}
