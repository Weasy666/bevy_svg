use bevy::{
    asset::RenderAssetUsages,
    color::{Alpha, Color, ColorToComponents},
    mesh::{Indices, Mesh, MeshVertexAttribute, VertexAttributeValues},
    render::render_resource::{PrimitiveTopology, VertexFormat},
};
use copyless::VecHelper;
use lyon_path::math::Point;
use lyon_tessellation::{
    self, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};

use crate::{Convert, gradient::PathPaint};

/// Position of the vertex in the coordinate system of its gradient.
pub const ATTRIBUTE_GRADIENT_POSITION: MeshVertexAttribute = MeshVertexAttribute::new(
    "SvgGradientPosition",
    0x5f0d_8a3c_29e1_4b67,
    VertexFormat::Float32x2,
);

/// Row of the vertex's gradient in [`Svg::gradient_image`](crate::svg::Svg::gradient_image)
/// plus one, or [`NO_GRADIENT`] for solid colors.
pub const ATTRIBUTE_GRADIENT_INDEX: MeshVertexAttribute = MeshVertexAttribute::new(
    "SvgGradientIndex",
    0x5f0d_8a3c_29e1_4b68,
    VertexFormat::Uint32,
);

pub const NO_GRADIENT: u32 = 0;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: [f32; 3],
    /// The fill color, or the tint that is multiplied with the gradient.
    color: [f32; 4],
    gradient_position: [f32; 2],
    gradient_index: u32,
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
pub type IndexType = u32;

/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub type VertexBuffers = lyon_tessellation::VertexBuffers<Vertex, IndexType>;

#[allow(clippy::single_call_fn)]
fn flip_mesh_vertically(mesh: &mut Mesh) {
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for position in positions.iter_mut() {
            // Invert the y-coordinate to flip the mesh vertically
            position[1] = -position[1];
        }
    }
}

impl Convert<Mesh> for VertexBuffers {
    fn convert(self) -> Mesh {
        let mut positions = Vec::with_capacity(self.vertices.len());
        let mut colors = Vec::with_capacity(self.vertices.len());
        let mut gradient_positions = Vec::with_capacity(self.vertices.len());
        let mut gradient_indices = Vec::with_capacity(self.vertices.len());

        for vert in self.vertices {
            positions.alloc().init(vert.position);
            colors.alloc().init(vert.color);
            gradient_positions.alloc().init(vert.gradient_position);
            gradient_indices.alloc().init(vert.gradient_index);
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_attribute(ATTRIBUTE_GRADIENT_POSITION, gradient_positions);
        mesh.insert_attribute(ATTRIBUTE_GRADIENT_INDEX, gradient_indices);
        mesh.insert_indices(Indices::U32(self.indices));

        // Bevy has a different y-axis origin, so we need to flip that axis
        flip_mesh_vertically(&mut mesh);

        mesh
    }
}

/// Zero-sized type used to implement various vertex construction traits from Lyon.
pub struct VertexConstructor {
    pub(crate) paint: PathPaint,
    pub(crate) transform: usvg::Transform,
}

impl VertexConstructor {
    fn process_vertex(&self, point: Point) -> Vertex {
        let local = usvg::tiny_skia_path::Point::from_xy(point.x, point.y);
        let mut pos = local;
        self.transform.map_point(&mut pos);

        let (color, gradient_position, gradient_index) = match self.paint {
            PathPaint::Color(color) => (color, [0.0; 2], NO_GRADIENT),
            PathPaint::Gradient {
                index,
                opacity,
                local_to_gradient,
            } => {
                let mut gradient_pos = local;
                local_to_gradient.map_point(&mut gradient_pos);
                (
                    Color::WHITE.with_alpha(opacity),
                    [gradient_pos.x, gradient_pos.y],
                    index + 1,
                )
            }
        };
        Vertex {
            position: [pos.x, pos.y, 0.0],
            color: color.to_linear().to_f32_array(),
            gradient_position,
            gradient_index,
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        self.process_vertex(vertex.position())
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        self.process_vertex(vertex.position())
    }
}

pub trait BufferExt<A> {
    fn extend_one(&mut self, item: A);
    #[allow(dead_code)]
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}

impl BufferExt<VertexBuffers> for VertexBuffers {
    fn extend_one(&mut self, item: VertexBuffers) {
        let offset = self.vertices.len() as u32;

        for vert in item.vertices {
            self.vertices.alloc().init(vert);
        }
        for idx in item.indices {
            self.indices.alloc().init(idx + offset);
        }
    }

    fn extend<T: IntoIterator<Item = VertexBuffers>>(&mut self, iter: T) {
        let mut offset = self.vertices.len() as u32;

        for buf in iter {
            let num_verts = buf.vertices.len() as u32;
            for vert in buf.vertices {
                self.vertices.alloc().init(vert);
            }
            for idx in buf.indices {
                self.indices.alloc().init(idx + offset);
            }
            offset += num_verts;
        }
    }
}
