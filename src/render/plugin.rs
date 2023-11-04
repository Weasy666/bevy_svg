use crate::resources::{FillTessellator, StrokeTessellator};
use bevy::app::{App, Plugin};

#[cfg(feature = "2d")]
use crate::render::svg2d;
#[cfg(feature = "3d")]
use crate::render::svg3d;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = FillTessellator::default();
        let stroke_tess = StrokeTessellator::default();
        app.insert_resource(fill_tess).insert_resource(stroke_tess);

        #[cfg(feature = "2d")]
        app.add_plugins(svg2d::RenderPlugin);

        #[cfg(feature = "3d")]
        app.add_plugins(svg3d::RenderPlugin);
    }
}
