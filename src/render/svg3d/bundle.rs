//! Bevy [`Bundle`] representing an SVG entity.

use crate::origin::Origin;
use bevy::{
    ecs::bundle::Bundle,
    mesh::Mesh3d,
    prelude::{InheritedVisibility, ViewVisibility, Visibility},
    transform::components::{GlobalTransform, Transform},
};

use super::Svg3d;

/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle, Default)]
#[deprecated(
    since = "0.15.0",
    note = "Use the `Svg3d` component instead. Inserting `Svg3d` will also insert the other components required automatically."
)]
pub struct Svg3dBundle {
    pub svg: Svg3d,
    pub mesh: Mesh3d,
    /// [`Origin`] of the coordinate system and as such the origin for the Bevy position.
    pub origin: Origin,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
