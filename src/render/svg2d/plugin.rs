use crate::{
    origin::Origin,
    render::{
        Svg2d,
        svg2d::SVG_2D_SHADER_HANDLE,
        vertex_buffer::{ATTRIBUTE_GRADIENT_INDEX, ATTRIBUTE_GRADIENT_POSITION},
    },
    svg::Svg,
};
use bevy::{
    app::{App, Plugin},
    asset::{AssetApp, load_internal_asset},
    mesh::{Mesh, MeshVertexBufferLayoutRef},
    render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError},
    shader::{Shader, ShaderRef},
    sprite_render::{AlphaMode2d, Material2d, Material2dKey, Material2dPipeline, Material2dPlugin},
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
    fn vertex_shader() -> ShaderRef {
        SVG_2D_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        SVG_2D_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn specialize(
        _pipeline: &Material2dPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
            ATTRIBUTE_GRADIENT_POSITION.at_shader_location(2),
            ATTRIBUTE_GRADIENT_INDEX.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
