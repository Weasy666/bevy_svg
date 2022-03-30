use bevy::{
    asset::{Assets, Handle},
    core_pipeline::Transparent3d,
    ecs::{entity::Entity, query::With, world::{FromWorld, World}, system::{Query, Res, ResMut},},
    log::debug,
    math::{Vec3, Vec3Swizzles},
    pbr::MeshUniform,
    render::{
        mesh::Mesh,
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace,
            MultisampleState, PolygonMode, PrimitiveState, PipelineCache,
            RenderPipelineDescriptor, Shader, SpecializedRenderPipeline, SpecializedRenderPipelines, TextureFormat,
            VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
        },
        texture::BevyDefault,
        view::{ComputedVisibility, Msaa}, RenderWorld,
    },
    transform::components::GlobalTransform, pbr::{MeshPipeline, MeshPipelineKey, SetMeshViewBindGroup, SetMeshBindGroup, DrawMesh},
};
use copyless::VecHelper;

use crate::{render::SVG_3D_SHADER_HANDLE, svg::{Origin, Svg}};


#[derive(Default)]
pub struct ExtractedSvgs3d {
    svgs: Vec<ExtractedSvg3d>,
}

#[derive(Clone)]
pub struct ExtractedSvg3d {
    pub entity: Entity,
    pub mesh3d_handle: Handle<Mesh>,
    pub origin_offset: Vec3,
    pub z: f32,
}

/// Extract [`Svg`]s with a [`Handle`] to a [`Mesh`] component into [`RenderWorld`].
pub fn extract_svg_3d(
    mut render_world: ResMut<RenderWorld>,
    svgs: Res<Assets<Svg>>,
    query: Query<(Entity, &ComputedVisibility, &Handle<Svg>, &Handle<Mesh>, &Origin, &GlobalTransform), With<Handle<Svg>>>,
) {
    debug!("Extracting `Svg`s from `World`.");
    let mut extracted_svgs = render_world.get_resource_mut::<ExtractedSvgs3d>().unwrap();
    extracted_svgs.svgs.clear();
    for (entity, computed_visibility, svg_handle, mesh3d_handle, origin, global_transform) in query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }

        if let Some(svg) = svgs.get(svg_handle) {
            let mut transform = global_transform.clone();
            let scaled_size = svg.size * transform.scale.xy();
            transform.translation += origin.compute_translation(scaled_size);

            extracted_svgs.svgs.alloc().init(ExtractedSvg3d {
                entity,
                mesh3d_handle: mesh3d_handle.clone(),
                origin_offset: origin.compute_translation(scaled_size),
                z: global_transform.translation.z,
            });
        }
    }

    debug!("Extracted {} `Svg3d`s from `World` and inserted them into `RenderWorld`.", extracted_svgs.svgs.len());
}

pub fn prepare_svg_3d(
    svgs_2d: ResMut<ExtractedSvgs3d>,
    mut query: Query<(Entity, &mut MeshUniform)>,
) {
    for svg2d in &svgs_2d.svgs {
        // Get the Uniform (aka data that will be send to the shader) and apply the origin offset
        // to the translation.
        if let Ok((_, mut uniform)) = query.get_mut(svg2d.entity) {
            let column = uniform.transform.col_mut(3);
            column.x += svg2d.origin_offset.x;
            column.y += svg2d.origin_offset.y;
            column.z += svg2d.origin_offset.z;
        }
    }
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
    svgs_3d: ResMut<ExtractedSvgs3d>,
    mut views: Query<&mut RenderPhase<Transparent3d>>,
) {
    if svgs_3d.svgs.is_empty() {
        debug!("No `Svg3d`s found to queue.");
        return;
    }
    debug!("Queuing {} `Svg3d`s for drawing/rendering.", svgs_3d.svgs.len());
    let mesh_key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    let draw_svg_3d = transparent_draw_functions
        .read()
        .get_id::<DrawSvg3d>()
        .unwrap();

    // Iterate each view (a camera is a view)
    for mut transparent_phase in views.iter_mut() {
        // Queue all entities visible to that view
        for svg3d in &svgs_3d.svgs {
            // Get our specialized pipeline
            let mut mesh3d_key = mesh_key;
            if let Some(mesh) = render_meshes.get(&svg3d.mesh3d_handle) {
                mesh3d_key |= MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
            }

            let pipeline_id = pipelines.specialize(&mut pipeline_cache, &svg_3d_pipeline, mesh3d_key);
            transparent_phase.add(Transparent3d {
                entity: svg3d.entity,
                draw_function: draw_svg_3d,
                pipeline: pipeline_id,
                distance: svg3d.z,
            });
        }
    }
}

/// Specifies how to render a [`Svg`] in 2d.
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

// Pipeline for 2d [`Svg`]s.
pub struct Svg3dPipeline {
    mesh3d_pipeline: MeshPipeline,
}

impl FromWorld for Svg3dPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh3d_pipeline: MeshPipeline::from_world(world),
        }
    }
}

// Specializie the `Mesh2dPipeline` to draw [`Svg`]s in 2D.
impl SpecializedRenderPipeline for Svg3dPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Meshes for our Svgs only have position and color
        let formats = vec![
            // Position
            VertexFormat::Float32x3,
            // Color
            VertexFormat::Uint32,
        ];

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: SVG_3D_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                // Use our custom vertex buffer
                buffers: vec![VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats)],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: SVG_3D_SHADER_HANDLE.typed::<Shader>(),
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
        }
    }
}
