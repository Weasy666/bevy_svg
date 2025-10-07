use super::SVG_3D_SHADER_HANDLE;
use crate::svg::Svg;
use bevy::{
    app::{App, Plugin},
    asset::{AssetApp, load_internal_asset},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin},
    render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError},
    shader::{Shader, ShaderRef},
};

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SVG_3D_SHADER_HANDLE, "svg_3d.wgsl", Shader::from_wgsl);

        app.add_plugins(MaterialPlugin::<Svg>::default())
            .register_asset_reflect::<Svg>();
    }
}

impl Material for Svg {
    fn fragment_shader() -> ShaderRef {
        SVG_3D_SHADER_HANDLE.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> bevy::prelude::Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;

        Ok(())
    }
}
