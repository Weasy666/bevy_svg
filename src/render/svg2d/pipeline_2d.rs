use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Local, Query, Res, ResMut},
        world::{FromWorld, World},
    },
    log::debug,
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
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
    utils::FloatOrd,
};
use copyless::VecHelper;

use crate::render::svg2d::SVG_2D_SHADER_HANDLE;

#[derive(Clone, Component)]
pub struct ExtractedSvg2d(Mesh2dHandle);

/// Extract [`Svg`]s with a [`Mesh2dHandle`] component into [`RenderWorld`].
pub fn extract_svg_2d(
    mut commands: Commands,
    mut extracted_svgs: Local<Vec<(Entity, ExtractedSvg2d)>>,
    query: Extract<Query<(Entity, &ComputedVisibility, &Mesh2dHandle)>>,
) {
    debug!("Extracting `Svg`s from `World`.");
    for (entity, computed_visibility, mesh2d_handle) in query.iter() {
        if !computed_visibility.is_visible() {
            continue;
        }

        extracted_svgs
            .alloc()
            .init((entity, ExtractedSvg2d(mesh2d_handle.clone())));
    }

    debug!(
        "Extracted {} `Svg2d`s from `World` and inserted them into `RenderWorld`.",
        extracted_svgs.len()
    );
    commands.insert_or_spawn_batch(extracted_svgs.to_vec());
    extracted_svgs.clear();
}

/// Queue all extraced 2D [`Svg`]s for rendering with the [`Svg2dPipeline`] custom pipeline and [`DrawSvg2d`] draw function
#[allow(clippy::too_many_arguments)]
pub fn queue_svg_2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    svg_2d_pipeline: Res<Svg2dPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<Svg2dPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    svgs_2d: Query<(&ExtractedSvg2d, &Mesh2dUniform)>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<Transparent2d>,
    )>,
) {
    if svgs_2d.is_empty() {
        debug!("No `Svg2d`s found to queue.");
        return;
    }
    let draw_svg_2d = transparent_draw_functions
        .read()
        .get_id::<DrawSvg2d>()
        .unwrap();

    let mut num_svgs = 0;
    // Iterate each view (a camera is a view)
    for (view, visible_entities, mut transparent_phase) in views.iter_mut() {
        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples)
            | Mesh2dPipelineKey::from_hdr(view.hdr);

        // Queue all entities visible to that view
        for visible_entity in &visible_entities.entities {
            if let Ok((extraced_svg, mesh2d_uniform)) = svgs_2d.get(*visible_entity) {
                // Get our specialized pipeline
                let mut mesh2d_key = mesh_key;
                if let Some(mesh) = render_meshes.get(&extraced_svg.0 .0) {
                    mesh2d_key |=
                        Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                }
                let pipeline_id =
                    pipelines.specialize(&mut pipeline_cache, &svg_2d_pipeline, mesh2d_key);
                num_svgs += 1;
                transparent_phase.add(Transparent2d {
                    entity: *visible_entity,
                    draw_function: draw_svg_2d,
                    pipeline: pipeline_id,
                    // The 2d render items are sorted according to their z value before rendering,
                    // in order to get correct transparency
                    sort_key: FloatOrd(mesh2d_uniform.transform.w_axis.z),
                    // This material is not batched
                    batch_range: None,
                });
            }
        }
    }
    debug!("Queued {} `Svg2d`s for drawing/rendering.", num_svgs);
}

/// Specifies how to render a [`Svg`] in 2d.
pub type DrawSvg2d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMesh2dBindGroup<1>,
    // Draw the mesh
    DrawMesh2d,
);

/// Pipeline for 2d [`Svg`]s.
#[derive(Resource)]
pub struct Svg2dPipeline {
    mesh2d_pipeline: Mesh2dPipeline,
}

impl FromWorld for Svg2dPipeline {
    fn from_world(world: &mut World) -> Self {
        return Self {
            mesh2d_pipeline: Mesh2dPipeline::from_world(world),
        };
    }
}

// Specializie the `Mesh2dPipeline` to draw [`Svg`]s in 2D.
impl SpecializedRenderPipeline for Svg2dPipeline {
    type Key = Mesh2dPipelineKey;

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
                shader: SVG_2D_SHADER_HANDLE.typed::<Shader>(),
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
                shader: SVG_2D_SHADER_HANDLE.typed::<Shader>(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: if key.contains(Mesh2dPipelineKey::HDR) {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            // Use the two standard uniforms for 2d meshes
            layout: Some(vec![
                // Bind group 0 is the view uniform
                self.mesh2d_pipeline.view_layout.clone(),
                // Bind group 1 is the mesh uniform
                self.mesh2d_pipeline.mesh_layout.clone(),
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
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("svg_2d_pipeline".into()),
        };
    }
}
