use bevy::{
    asset::Handle,
    core::FloatOrd,
    core_pipeline::Transparent2d,
    ecs::{entity::Entity, query::With, world::{FromWorld, World}, system::{Query, Res, ResMut},},
    log::info,
    render::{
        mesh::Mesh,
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace,
            MultisampleState, PolygonMode, PrimitiveState, RenderPipelineCache,
            RenderPipelineDescriptor, Shader, SpecializedPipeline, SpecializedPipelines, TextureFormat,
            VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
        },
        texture::BevyDefault,
        view::{ComputedVisibility, Msaa}, RenderWorld,
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
    transform::components::GlobalTransform,
};
use copyless::VecHelper;

use crate::{render::SVG_2D_SHADER_HANDLE, svg::Svg};


#[derive(Default)]
pub struct ExtractedSvgs2d {
    svgs: Vec<ExtractedSvg2d>,
}

#[derive(Clone)]
pub struct ExtractedSvg2d {
    pub entity: Entity,
    pub mesh2d_handle: Mesh2dHandle,
    pub global_transform: GlobalTransform
}

/// Extract [`Svg`]s with a [`Mesh2dHandle`] component into [`RenderWorld`].
pub fn extract_svg_2d(
    mut render_world: ResMut<RenderWorld>,
    query: Query<(Entity, &ComputedVisibility, &Mesh2dHandle, &GlobalTransform), With<Handle<Svg>>>,
) {
    info!("Extracting `Svg`s from `World`.");
    let mut extracted_svgs = render_world.get_resource_mut::<ExtractedSvgs2d>().unwrap();
    extracted_svgs.svgs.clear();
    for (entity, computed_visibility, mesh2d_handle, global_transform) in query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        extracted_svgs.svgs.alloc().init(ExtractedSvg2d {
            entity,
            mesh2d_handle: mesh2d_handle.clone(),
            global_transform: global_transform.clone(),
        });
    }

    info!("Extracted {} `Svg2d`s from `World` and inserted them into `RenderWorld`.", extracted_svgs.svgs.len());
}

/// Queue all extraced 2D [`Svg`]s for rendering with the [`Svg2dPipeline`] custom pipeline and [`DrawSvg2d`] draw function
#[allow(clippy::too_many_arguments)]
pub fn queue_svg_2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    svg_2d_pipeline: Res<Svg2dPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<Svg2dPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    svgs_2d: ResMut<ExtractedSvgs2d>,
    mut views: Query<&mut RenderPhase<Transparent2d>>,
) {
    if svgs_2d.svgs.is_empty() {
        info!("No `Svg2d`s found to queue.");
        return;
    }
    info!("Queuing {} `Svg2d`s for drawing/rendering.", svgs_2d.svgs.len());
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

            let pipeline_id = pipelines.specialize(&mut pipeline_cache, &svg_2d_pipeline, mesh2d_key);
            let mesh_z = svg2d.global_transform.translation.z;
            transparent_phase.add(Transparent2d {
                entity: svg2d.entity,
                draw_function: draw_svg_2d,
                pipeline: pipeline_id,
                // The 2d render items are sorted according to their z value before rendering,
                // in order to get correct transparency
                sort_key: FloatOrd(mesh_z),
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
impl SpecializedPipeline for Svg2dPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Meshes for our Svgs only have position and color
        let vertex_attributes = vec![
            // Position (GOTCHA! Vertex_Position isn't first in the buffer due to how Mesh sorts attributes (alphabetically))
            VertexAttribute {
                format: VertexFormat::Float32x3,
                // this offset is the size of the color attribute, which is stored first
                offset: 16,
                // position is available at location 0 in the shader
                shader_location: 0,
            },
            // Color
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 0,
                shader_location: 1,
            },
        ];
        // Sum of the size of position and color attributes (12 + 16 = 28)
        let vertex_array_stride = 28;

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: SVG_2D_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                // Use our custom vertex buffer
                buffers: vec![VertexBufferLayout {
                    array_stride: vertex_array_stride,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vertex_attributes,
                }],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: SVG_2D_SHADER_HANDLE.typed::<Shader>(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                }],
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
