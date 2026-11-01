use bevy::log::{debug, error};
use lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, StrokeTessellator};

use crate::{
    render::vertex_buffer::{BufferExt, VertexBuffers, VertexConstructor},
    svg::{DrawType, Svg},
};

pub fn generate_buffer(
    svg: &Svg,
    fill_tess: &mut FillTessellator,
    stroke_tess: &mut StrokeTessellator,
) -> VertexBuffers {
    debug!("Tessellating SVG: {}", svg.name);

    let mut buffers = VertexBuffers::new();

    for path in &svg.paths {
        let mut buffer = VertexBuffers::new();

        let segments = path.segments.clone();
        let vertex_constructor = VertexConstructor {
            paint: path.paint,
            transform: path.abs_transform,
        };
        match path.draw_type {
            DrawType::Fill => {
                if let Err(e) = fill_tess.tessellate(
                    segments,
                    &FillOptions::tolerance(0.001),
                    &mut BuffersBuilder::new(&mut buffer, vertex_constructor),
                ) {
                    error!("FillTessellator error: {:?}", e);
                }
            }
            DrawType::Stroke(opts) => {
                if let Err(e) = stroke_tess.tessellate(
                    segments,
                    &opts,
                    &mut BuffersBuilder::new(&mut buffer, vertex_constructor),
                ) {
                    error!("StrokeTessellator error: {:?}", e);
                }
            }
        }
        buffers.extend_one(buffer);
    }
    debug!("Tessellating SVG: {} ... Done", svg.name);

    buffers
}
