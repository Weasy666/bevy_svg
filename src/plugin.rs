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

use crate::svg::Svg;
use bevy::{
    app::{AppBuilder, Plugin}, asset::{Assets, Handle}, ecs::{Added, IntoSystem, Query, ResMut, SystemStage}, log::error,
    prelude::{Color, ColorMaterial}, render::{draw::Visible, mesh::{Indices, Mesh}, pipeline::PrimitiveTopology,},
};
use lyon_tessellation::{
    self, BuffersBuilder, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeTessellator, StrokeVertex, StrokeVertexConstructor,
};

/// Stages for this plugin.
pub mod stage {
    /// Stage in which [`SvgBundle`](crate::bundle::SvgBundle)s get converted into drawable meshes.
    pub const SVG: &str = "svg";
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
}

/// Zero-sized type used to implement various vertex construction traits from Lyon.
struct VertexConstructor;

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
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
                bevy::app::stage::UPDATE,
                stage::SVG,
                SystemStage::parallel(),
            )
            .add_system_to_stage(stage::SVG, svg_mesh_maker.system());
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

        //TODO: Try to create a Texture with the colors specified in the SVG file and map the colors to
        // the meshs UV coords...if that is possible
        // let width;
        // let height;
        // let raw_data = i.into_raw();
        // let texture = Texture::new(
        //     Extent3d::new(width, height, 1),
        //     TextureDimension::D2,
        //     raw_data.as_slice().as_bytes().to_owned(),
        //     TextureFormat::Rgba16Uint,
        // );
        let mut color = None;
        for path in svg.paths.iter() {
            if let Some(stroke) = path.style.stroke.or(svg.style.stroke) {
                color = stroke.color.map(|c|
                        Color::rgb_u8(c.red, c.green, c.blue)
                            .set_a(stroke.opacity.unwrap_or(1.0)).to_owned()
                    )
                    .or(color);
                if let Err(e) = stroke_tess.tessellate_path(
                    path.d.as_slice(),
                    &stroke.to_options(),
                    &mut BuffersBuilder::new(&mut buffers, VertexConstructor)
                ) {
                    error!("StrokeTessellator error: {:?}", e)
                }
            }

            if let Some(fill) = path.style.fill.or(svg.style.fill) {
                color = fill.color.map(|c|
                        Color::rgb_u8(c.red, c.green, c.blue)
                            .set_a(fill.opacity.unwrap_or(1.0)).to_owned()
                    )
                    .or(color);
                if let Err(e) = fill_tess.tessellate_path(
                    path.d.as_slice(),
                    &fill.to_options(),
                    &mut BuffersBuilder::new(&mut buffers, VertexConstructor)
                ) {
                    error!("FillTessellator error: {:?}", e)
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

    mesh
}
