//! Bevy [`Bundle`] representing an SVG entity.

use bevy::{
    camera::visibility::{InheritedVisibility, ViewVisibility, Visibility},
    ecs::bundle::Bundle,
    mesh::Mesh2d,
    transform::components::{GlobalTransform, Transform},
};

use crate::origin::Origin;

use super::Svg2d;

/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle, Default)]
#[deprecated(
    since = "0.15.0",
    note = "Use the `Svg2d` component instead. Inserting `Svg2d` will also insert the other components required automatically."
)]
pub struct Svg2dBundle {
    pub svg: Svg2d,
    pub mesh_2d: Mesh2d,
    /// [`Origin`] of the coordinate system and as such the origin for the Bevy position.
    pub origin: Origin,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
