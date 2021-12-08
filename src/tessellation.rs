use bevy::{prelude::{error, Transform, info}, math::Vec3};
use lyon_tessellation::{FillTessellator, StrokeTessellator, FillOptions, BuffersBuilder};

use crate::{prelude::Svg, vertex_buffer::{VertexBuffers, VertexConstructor, BufferExt}, svg::DrawType};


pub(crate) fn generate_buffer(
    svg: &Svg,
    fill_tess: &mut FillTessellator,
    stroke_tess: &mut StrokeTessellator,
) -> VertexBuffers {
    info!("Tessellating SVG: {}", svg.name);

    let flip_y = Transform::from_scale(Vec3::new(1.0, -1.0, 1.0));
    let mut buffers = VertexBuffers::new();

    let mut color = None;
    for path in &svg.paths {
        let mut buffer = VertexBuffers::new();

        if color.is_none() {
            color = Some(path.color);
        }

        match path.draw_type {
            DrawType::Fill => {
                if let Err(e) = fill_tess.tessellate(
                    path.segments.clone(),
                    &FillOptions::tolerance(0.001),
                    &mut BuffersBuilder::new(&mut buffer, VertexConstructor { color: path.color })
                ) {
                    error!("FillTessellator error: {:?}", e)
                }
            },
            DrawType::Stroke(opts) => {
                if let Err(e) = stroke_tess.tessellate(
                    path.segments.clone(),
                    &opts,
                    &mut BuffersBuilder::new(&mut buffer, VertexConstructor { color: path.color })
                ) {
                    error!("StrokeTessellator error: {:?}", e)
                }
            }
        }

        // Bevy has a different y-axis origin, so we need to flip that axis
        buffer.apply_transform(flip_y * path.abs_transform);
        buffers.extend_one(buffer);
    }
    info!("Tessellating SVG: {} ... Done", svg.name);

    buffers
}
