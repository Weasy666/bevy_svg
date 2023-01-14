use bevy::{
    asset::Handle,
    core_pipeline::core_3d::Transparent3d,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Local, Query, Res, ResMut},
        world::{FromWorld, World},
    },
    log::debug,
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::Resource,
    render::{
        mesh::Mesh,
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace, MultisampleState,
            PipelineCache, PolygonMode, PrimitiveState, RenderPipelineDescriptor, Shader,
            SpecializedRenderPipeline, SpecializedRenderPipelines, TextureFormat,
            VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
        },
        texture::BevyDefault,
        view::{ComputedVisibility, ExtractedView, Msaa, ViewTarget, VisibleEntities},
        Extract,
    },
};
use copyless::VecHelper;

use crate::{render::svg3d::SVG_3D_SHADER_HANDLE, svg::Svg};

#[derive(Clone, Component)]
pub struct ExtractedSvg3d(Handle<Mesh>);

/// Extract [`Svg`]s with a [`Handle`] to a [`Mesh`] component into [`RenderWorld`].
pub fn extract_svg_3d(
    mut commands: Commands,
    mut extracted_svgs: Local<Vec<(Entity, ExtractedSvg3d)>>,
    query: Extract<Query<(Entity, &ComputedVisibility, &Handle<Mesh>), With<Handle<Svg>>>>,
) {
    debug!("Extracting `Svg`s from `World`.");
    for (entity, computed_visibility, mesh3d_handle) in query.iter() {
        if !computed_visibility.is_visible() {
            continue;
        }

        extracted_svgs
            .alloc()
            .init((entity, ExtractedSvg3d(mesh3d_handle.clone())));
    }

    debug!(
        "Extracted {} `Svg3d`s from `World` and inserted them into `RenderWorld`.",
        extracted_svgs.len()
    );
    commands.insert_or_spawn_batch(extracted_svgs.to_vec());
    extracted_svgs.clear();
}

/// Queue all extraced 3D [`Svg`]s for rendering with the [`Svg3dPipeline`] custom pipeline and [`DrawSvg3d`] draw function
#[allow(clippy::too_many_arguments)]
pub fn queue_svg_3d(
    transparent_draw_functions: Res<DrawFunctions<Transparent3d>>,
    svg_3d_pipeline: Res<Svg3dPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<Svg3dPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    svgs_3d: Query<(&ExtractedSvg3d, &MeshUniform)>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<Transparent3d>,
    )>,
) {
    if svgs_3d.is_empty() {
        debug!("No `Svg3d`s found to queue.");
        return;
    }
    let draw_svg_3d = transparent_draw_functions
        .read()
        .get_id::<DrawSvg3d>()
        .unwrap();

    let mut num_svgs = 0;
    // Iterate each view (a camera is a view)
    for (view, visible_entities, mut transparent_phase) in views.iter_mut() {
        let mesh_key =
            MeshPipelineKey::from_msaa_samples(msaa.samples) | MeshPipelineKey::from_hdr(view.hdr);

        // Queue all entities visible to that view
        for visible_entity in &visible_entities.entities {
            if let Ok((extraced_svg, mesh_uniform)) = svgs_3d.get(*visible_entity) {
                // Get our specialized pipeline
                let mut mesh3d_key = mesh_key;
                if let Some(mesh) = render_meshes.get(&extraced_svg.0) {
                    mesh3d_key |= MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                }
                let pipeline_id =
                    pipelines.specialize(&mut pipeline_cache, &svg_3d_pipeline, mesh3d_key);
                num_svgs += 1;
                transparent_phase.add(Transparent3d {
                    entity: *visible_entity,
                    draw_function: draw_svg_3d,
                    pipeline: pipeline_id,
                    distance: mesh_uniform.transform.w_axis.z,
                });
            }
        }
    }
    debug!("Queued {} `Svg3d`s for drawing/rendering.", num_svgs);
}

/// Specifies how to render a [`Svg`] in 3d.
pub type DrawSvg3d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMeshViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMeshBindGroup<1>,
    // Draw the mesh
    DrawMesh,
);

// Pipeline for 3d [`Svg`]s.
#[derive(Resource)]
pub struct Svg3dPipeline {
    mesh3d_pipeline: MeshPipeline,
}

impl FromWorld for Svg3dPipeline {
    fn from_world(world: &mut World) -> Self {
        return Self {
            mesh3d_pipeline: MeshPipeline::from_world(world),
        };
    }
}

// Specializie the `MeshPipeline` to draw [`Svg`]s in 3D.
impl SpecializedRenderPipeline for Svg3dPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Meshes for our Svgs only have position and color
        let formats = vec![
            // Position
            VertexFormat::Float32x3,
            // Color
            VertexFormat::Float32x4,
        ];

        return RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: SVG_3D_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                // Use our custom vertex buffer
                buffers: vec![VertexBufferLayout::from_vertex_formats(
                    VertexStepMode::Vertex,
                    formats,
                )],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: SVG_3D_SHADER_HANDLE.typed::<Shader>(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: if key.contains(MeshPipelineKey::HDR) {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            // Use the two standard uniforms for 3d meshes
            layout: Some(vec![
                // Bind group 0 is the view uniform
                self.mesh3d_pipeline.view_layout.clone(),
                // Bind group 1 is the mesh uniform
                self.mesh3d_pipeline.mesh_layout.clone(),
            ]),
            primitive: PrimitiveState {
                front_face: FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: key.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil: Some(bevy::render::render_resource::DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: bevy::render::render_resource::CompareFunction::Greater,
                stencil: bevy::render::render_resource::StencilState {
                    front: bevy::render::render_resource::StencilFaceState::IGNORE,
                    back: bevy::render::render_resource::StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: bevy::render::render_resource::DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("svg_3d_pipeline".into()),
        };
    }
}
