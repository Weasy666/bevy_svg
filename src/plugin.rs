//! Contains the plugin and its helper types.
//!
//! The [`SvgBundle`] provides the creation of shapes with minimal
//! boilerplate.
//!
//! ## How it works
//! The user spawns a [`SvgBundle`](crate::bundle::SvgBundle) from a
//! system in the [`UPDATE`](bevy::app::stage::UPDATE) stage.
//!
//! Then, in the [`SVG`](stage::SVG) stage, there is a system
//! that creates a mesh for each entity that has been spawned as a
//! `SvgBundle`.

use crate::{Convert, svg::Svg, tessellation};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AddAsset, Assets, Handle, HandleUntyped},
    ecs::{
        query::Added,
        schedule::{StageLabel, SystemStage},
        system::{IntoSystem, Query, ResMut}
    },
    reflect::TypeUuid,
    render::{
        mesh::Mesh,
        pipeline::PipelineDescriptor,
        render_graph::{AssetRenderResourcesNode, base, RenderGraph},
        renderer::RenderResources,
        shader::{Shader, ShaderStage, ShaderStages}
    },
};
use lyon_tessellation::{FillTessellator, StrokeTessellator};

pub const SVG_PIPELINE_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 8514826620251853414);

/// Stages for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum Stage {
    /// Stage in which [`SvgBundle`](crate::bundle::SvgBundle)s get converted into drawable meshes.
    SVG,
}

/// A plugin that provides resources and a system to draw [`SvgBundle`]s in Bevy with..
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app.add_asset::<SvgMaterial>()
            .insert_resource(fill_tess)
            .insert_resource(stroke_tess)
            .add_startup_system(setup.system())
            .add_stage_after(
                bevy::app::CoreStage::Update,
                Stage::SVG,
                SystemStage::parallel(),
            )
            .add_system_to_stage(Stage::SVG, svg_mesh_maker.system());
    }
}

fn setup(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    pipelines.set_untracked(
        SVG_PIPELINE_HANDLE,
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
        })
    );

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterialWithVertexColorSupport resources to our shader
    render_graph.add_system_node(
        "svg_material",
        AssetRenderResourcesNode::<SvgMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge(
            "svg_material",
            base::node::MAIN_PASS,
        )
        .unwrap();
}

/// Bevy system which queries all [`SvgBundle`]s to complete them with a mesh and material.
fn svg_mesh_maker(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (&Svg, &mut Handle<Mesh>),
        Added<Svg>
    >,
) {
    for (svg, mut mesh) in query.iter_mut() {
        let buffer = tessellation::generate_buffer(&svg, &mut fill_tess, &mut stroke_tess);
        *mesh = meshes.add(buffer.convert());
    }
            }

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "d2c5985d-e221-4257-9e3b-ff0fb87e28ba"]
pub struct SvgMaterial;

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;

layout(location = 0) out vec4 v_color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_color = Vertex_Color;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = v_color;
}
"#;
