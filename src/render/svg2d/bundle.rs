//! Bevy [`Bundle`] representing an SVG entity.

use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    render::view::{ComputedVisibility, Visibility},
    sprite::Mesh2dHandle,
    transform::components::{GlobalTransform, Transform},
};

use crate::svg::{Origin, Svg};

/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct Svg2dBundle {
    pub svg: Handle<Svg>,
    pub mesh_2d: Mesh2dHandle,
    /// [`Origin`] of the coordinate system and as such the origin for the Bevy position.
    pub origin: Origin,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for Svg2dBundle {
    /// Creates a default [`Svg2dBundle`].
    fn default() -> Self {
        Self {
            svg: Default::default(),
            mesh_2d: Default::default(),
            origin: Default::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}
