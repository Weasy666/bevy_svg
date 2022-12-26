use crate::resources::{FillTessellator, StrokeTessellator};
use bevy::{
    app::{App, Plugin},
    asset::Assets,
    render::render_resource::Shader,
    render::RenderApp,
};

#[cfg(feature = "2d")]
use crate::render::svg2d;
#[cfg(feature = "3d")]
use crate::render::svg3d;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        // Load SVG shader
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        #[cfg(feature = "2d")]
        shaders.set_untracked(
            svg2d::SVG_2D_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("svg2d/svg_2d.wgsl")),
        );
        #[cfg(feature = "3d")]
        shaders.set_untracked(
            svg3d::SVG_3D_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("svg3d/svg_3d.wgsl")),
        );
        let fill_tess = FillTessellator::default();
        let stroke_tess = StrokeTessellator::default();
        app.insert_resource(fill_tess).insert_resource(stroke_tess);
        // Register our custom draw function and pipeline, and add our render systems
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            #[cfg(feature = "2d")]
            render_app.add_plugin(svg2d::RenderPlugin);
            #[cfg(feature = "3d")]
            render_app.add_plugin(svg3d::RenderPlugin);
        }
    }
}
