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

use crate::{Convert, svg::Svg, tessellation, loader::SvgAssetLoader, prelude::Origin};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AddAsset, Assets, Handle, HandleUntyped},
    ecs::{
        schedule::{StageLabel, SystemStage},
        system::{IntoSystem, Query, Res, ResMut}
    },
    reflect::TypeUuid,
    render::{
        mesh::Mesh,
        pipeline::PipelineDescriptor,
        shader::{Shader, ShaderStage, ShaderStages}
    }, prelude::{info, AssetEvent, EventReader, Entity, Transform}, math::Vec3,
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
        app
            .add_asset::<Svg>()
            .init_asset_loader::<SvgAssetLoader>()
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
) {
    // Create a new shader pipeline
    pipelines.set_untracked(
        SVG_PIPELINE_HANDLE,
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
        })
    );
}

/// Bevy system which queries all [`SvgBundle`]s to complete them with a mesh and material.
fn svg_mesh_maker(
    mut svg_events: EventReader<AssetEvent<Svg>>,
    svgs: Res<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (Entity, &Handle<Svg>, &mut Handle<Mesh>, &Origin, &mut Transform),
    >,
) {
    for event in svg_events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                let bundle = query.iter_mut().filter(|(_, svg, _, _, _)| svg == &handle).next();
                if let Some((_, _, mut mesh, origin, mut transform)) = bundle {
                    let svg = svgs.get(handle).unwrap();
                    let translation = match origin {
                        Origin::Center => transform.translation + Vec3::new(
                            -svg.width as f32 * transform.scale.x / 2.0,
                            svg.height as f32 * transform.scale.y / 2.0,
                            0.0
                        ),
                        Origin::TopLeft => transform.translation,
                    };
                    transform.translation = translation;

                    info!("Make mesh for SVG: {}", svg.name);
                    let buffer = tessellation::generate_buffer(&svg, &mut fill_tess, &mut stroke_tess);
                    *mesh = meshes.add(buffer.convert());
                }
            },
            AssetEvent::Removed { handle } => {
                let _bundle = query.iter_mut().filter(|(_, svg, _, _, _)| svg == &handle).next();
                //TODO:
            },
        }
    }
}

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
