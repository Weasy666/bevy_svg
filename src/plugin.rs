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

use crate::svg::{DrawType, Svg};
use bevy::{
    app::{AppBuilder, Plugin}, asset::{Assets, Handle}, ecs::{Added, IntoSystem, Query, ResMut, StageLabel, SystemStage}, log::error,
    prelude::{Color, ColorMaterial}, render::{draw::Visible, mesh::{Indices, Mesh}, pipeline::PrimitiveTopology,},
};
use lyon_tessellation::{self, BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor, StrokeTessellator, StrokeVertex, StrokeVertexConstructor};

/// Stages for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum Stage {
    /// Stage in which [`SvgBundle`](crate::bundle::SvgBundle)s get converted into drawable meshes.
    SVG,
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
type IndexType = u32;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
type VertexBuffers = lyon_tessellation::VertexBuffers<Vertex, IndexType>;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    color: [f32; 4],
}

/// Zero-sized type used to implement various vertex construction traits from Lyon.
struct VertexConstructor {
    color: Color,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
            color: [self.color.r(), self.color.g(), self.color.b(), self.color.a()],
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
            color: [self.color.r(), self.color.g(), self.color.b(), self.color.a()],
        }
    }
}

/// A plugin that provides resources and a system to draw [`SvgBundle`]s in Bevy with..
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app.insert_resource(fill_tess)
            .insert_resource(stroke_tess)
            .add_stage_after(
                bevy::app::CoreStage::Update,
                Stage::SVG,
                SystemStage::parallel(),
            )
            .add_system_to_stage(Stage::SVG, svg_mesh_maker.system());
    }
}

/// Bevy system which queries all [`SvgBundle`]s to complete them with a mesh and material.
fn svg_mesh_maker(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (&Svg, &mut Handle<Mesh>, &mut Handle<ColorMaterial>, &mut Visible),
        Added<Svg>
    >,
) {
    for (svg, mut mesh, mut material, mut visible) in query.iter_mut() {
        let mut buffers = VertexBuffers::new();

        //TODO: still need to do something about the color
        let mut color = None;
        for path in svg.paths.iter() {
            if color.is_none() {
                color = Some(path.color);
            }
            match path.draw_type {
                DrawType::Fill => {
                    if let Err(e) = fill_tess.tessellate(
                        path.segments.clone(),
                        &FillOptions::tolerance(0.001),
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor { color: path.color })
                    ) {
                        error!("FillTessellator error: {:?}", e)
                    }
                },
                DrawType::Stroke(opts) => {
                    if let Err(e) = stroke_tess.tessellate(
                        path.segments.clone(),
                        &opts,
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor { color: path.color })
                    ) {
                        error!("StrokeTessellator error: {:?}", e)
                    }
                }
            }
        }

        *material = materials.add(color.unwrap_or(Color::BLACK).into());
        *mesh = meshes.add(build_mesh(&buffers));
        visible.is_visible = true;
    }
}

fn build_mesh(buffers: &VertexBuffers) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| v.position)
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        buffers
            .vertices
            .iter()
            .map(|v| v.normal)
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        buffers
            .vertices
            .iter()
            .map(|v| v.uv)
            .collect::<Vec<[f32; 2]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        buffers
            .vertices
            .iter()
            .map(|v| v.color)
            .collect::<Vec<[f32; 4]>>(),
    );

    mesh
}
