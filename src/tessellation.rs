use bevy::{prelude::error};
use lyon_tessellation::{FillTessellator, StrokeTessellator, FillOptions, BuffersBuilder};

use crate::{prelude::Svg, vertex_buffer::{VertexBuffers, VertexConstructor, BufferExt}, svg::DrawType};


pub(crate) fn generate_buffer(
    svg: &Svg,
    fill_tess: &mut FillTessellator,
    stroke_tess: &mut StrokeTessellator,
) -> VertexBuffers {
    let mut buffers = VertexBuffers::new();

    //TODO: still need to do something about the color, it is pretty washed out
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

        buffer.apply_transform(path.abs_transform);
        buffers.extend_one(buffer);
    }

    buffers
}
