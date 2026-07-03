use crate::{
    origin::Origin,
    render::svg2d::{SVG_2D_SHADER_HANDLE, Svg2d},
    svg::Svg,
};
use bevy::{
    app::{App, Plugin},
    asset::{AssetApp, load_internal_asset},
    shader::{Shader, ShaderRef},
    sprite_render::{Material2d, Material2dPlugin},
};

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SVG_2D_SHADER_HANDLE, "svg_2d.wesl", Shader::from_wesl);

        app.add_plugins(Material2dPlugin::<Svg>::default())
            .register_asset_reflect::<Svg>()
            .register_type::<Svg2d>()
            .register_type::<Origin>();
    }
}

impl Material2d for Svg {
    fn fragment_shader() -> ShaderRef {
        SVG_2D_SHADER_HANDLE.into()
    }
}
