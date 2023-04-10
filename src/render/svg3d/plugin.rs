use bevy::{
    app::{App, Plugin},
    asset::{AddAsset, load_internal_asset},
    pbr::{Material, MaterialPlugin},
    render::render_resource::{Shader, ShaderRef},
};

use crate::svg::Svg;

use super::SVG_3D_SHADER_HANDLE;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SVG_3D_SHADER_HANDLE,
            "svg_3d.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(MaterialPlugin::<Svg>::default())
            .register_asset_reflect::<Svg>();
    }
}

impl Material for Svg {
    fn fragment_shader() -> ShaderRef {
        SVG_3D_SHADER_HANDLE.typed().into()
    }
}
