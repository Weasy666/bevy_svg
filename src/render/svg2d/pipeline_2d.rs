use bevy::{
    asset::{Assets, Handle},
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        entity::Entity,
        system::{Query, Res, ResMut},
        world::{FromWorld, World},
    },
    log::debug,
    math::{Vec3, Vec3Swizzles},
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
        view::{ComputedVisibility, Msaa},
        Extract,
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
    transform::components::Transform,
    utils::FloatOrd,
};
use copyless::VecHelper;

use crate::{
    origin::Origin,
    render::svg2d::SVG_2D_SHADER_HANDLE,
    svg::Svg,
};

#[derive(Default, Resource)]
pub struct ExtractedSvgs2d {
    svgs: Vec<ExtractedSvg2d>,
}

#[derive(Clone, Resource)]
pub struct ExtractedSvg2d {
    pub entity: Entity,
    pub mesh2d_handle: Mesh2dHandle,
    pub origin_offset: Vec3,
    pub z: f32,
}

/// Extract [`Svg`]s with a [`Mesh2dHandle`] component into [`RenderWorld`].
pub fn extract_svg_2d(
    mut extracted_svgs: ResMut<ExtractedSvgs2d>,
    svgs: Extract<Res<Assets<Svg>>>,
    query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &Handle<Svg>,
            &Mesh2dHandle,
            &Origin,
            &Transform,
        )>,
    >,
) {
    debug!("Extracting `Svg`s from `World`.");
    extracted_svgs.svgs.clear();
    for (entity, computed_visibility, svg_handle, mesh2d_handle, origin, transform) in query.iter()
    {
        if !computed_visibility.is_visible() {
            continue;
        }

        if let Some(svg) = svgs.get(svg_handle) {
            let mut transform = transform.clone();
            let scaled_size = svg.size * transform.scale.xy();
            transform.translation += origin.compute_translation(scaled_size);

            extracted_svgs.svgs.alloc().init(ExtractedSvg2d {
                entity,
                mesh2d_handle: mesh2d_handle.clone(),
                origin_offset: origin.compute_translation(scaled_size),
                z: transform.translation.z,
            });
        }
    }

    debug!(
        "Extracted {} `Svg2d`s from `World` and inserted them into `RenderWorld`.",
        extracted_svgs.svgs.len()
    );
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
    svgs_2d: ResMut<ExtractedSvgs2d>,
    mut views: Query<&mut RenderPhase<Transparent2d>>,
) {
    if svgs_2d.svgs.is_empty() {
        debug!("No `Svg2d`s found to queue.");
        return;
    }
    debug!(
        "Queuing {} `Svg2d`s for drawing/rendering.",
        svgs_2d.svgs.len()
    );
    let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples);
    let draw_svg_2d = transparent_draw_functions
        .read()
        .get_id::<DrawSvg2d>()
        .unwrap();

    // Iterate each view (a camera is a view)
    for mut transparent_phase in views.iter_mut() {
        // Queue all entities visible to that view
        for svg2d in &svgs_2d.svgs {
            // Get our specialized pipeline
            let mut mesh2d_key = mesh_key;
            if let Some(mesh) = render_meshes.get(&svg2d.mesh2d_handle.0) {
                mesh2d_key |= Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
            }

            let pipeline_id =
                pipelines.specialize(&mut pipeline_cache, &svg_2d_pipeline, mesh2d_key);
            transparent_phase.add(Transparent2d {
                entity: svg2d.entity,
                draw_function: draw_svg_2d,
                pipeline: pipeline_id,
                // The 2d render items are sorted according to their z value before rendering,
                // in order to get correct transparency
                sort_key: FloatOrd(svg2d.z),
                // This material is not batched
                batch_range: None,
            });
        }
    }
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
        Self {
            mesh2d_pipeline: Mesh2dPipeline::from_world(world),
        }
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

        RenderPipelineDescriptor {
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
                    format: TextureFormat::bevy_default(),
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
        }
    }
}
