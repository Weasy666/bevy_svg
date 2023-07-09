use crate::{
    prelude::Svg,
    resources::{FillTessellator, StrokeTessellator},
};
use bevy::{
    app::{App, Plugin},
    asset::{Assets, Handle},
};

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

        app.world
            .resource_mut::<Assets<Svg>>()
            .set_untracked(Handle::<Svg>::default(), Svg::default());

        #[cfg(feature = "2d")]
        app.add_plugins(svg2d::RenderPlugin);

        #[cfg(feature = "3d")]
        app.add_plugins(svg3d::RenderPlugin);
    }
}
