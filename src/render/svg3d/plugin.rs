use super::SVG_3D_SHADER_HANDLE;
use crate::{
    render::vertex_buffer::{ATTRIBUTE_GRADIENT_INDEX, ATTRIBUTE_GRADIENT_POSITION},
    svg::Svg,
};
use bevy::{
    app::{App, Plugin},
    asset::{AssetApp, load_internal_asset},
    mesh::{Mesh, MeshVertexBufferLayoutRef},
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin},
    prelude::AlphaMode,
    render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError},
    shader::{Shader, ShaderRef},
};

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SVG_3D_SHADER_HANDLE, "svg_3d.wesl", Shader::from_wesl);

        app.add_plugins(MaterialPlugin::<Svg>::default())
            .register_asset_reflect::<Svg>()
            .register_type::<Svg3d>()
            .register_type::<Origin>();
    }
}

impl Material for Svg {
    fn vertex_shader() -> ShaderRef {
        SVG_3D_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        SVG_3D_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> bevy::prelude::Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;

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
