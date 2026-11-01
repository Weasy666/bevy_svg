#[cfg(feature = "2d")]
use bevy::mesh::Mesh2d;
#[cfg(feature = "3d")]
use bevy::mesh::Mesh3d;
use bevy::{
    asset::{AssetId, Assets, Handle, StrongHandle},
    ecs::{
        component::Component,
        query::Changed,
        reflect::ReflectComponent,
        resource::Resource,
        system::{Query, Res, ResMut},
    },
    math::{FloatOrd, Vec2, Vec3},
    mesh::Mesh,
    platform::collections::HashMap,
    reflect::{Reflect, std_traits::ReflectDefault},
};

#[cfg(feature = "2d")]
use crate::render::Svg2d;
#[cfg(feature = "3d")]
use crate::render::Svg3d;

use crate::svg::Svg;
use std::sync::{Arc, Weak};

#[derive(Clone, Component, Copy, Debug, Default, PartialEq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq)]
/// Origin of the coordinate system.
pub enum Origin {
    /// Bottom left of the image or viewbox.
    BottomLeft,
    /// Bottom right of the image or viewbox.
    BottomRight,
    /// Center of the image or viewbox.
    Center,
    #[default]
    /// Top left of the image or viewbox, this is the default for a SVG.
    TopLeft,
    /// Top right of the image or viewbox.
    TopRight,
    /// Custom origin, top left is (0, 0), bottom right is (1, 1)
    Custom((f32, f32)),
}

impl Origin {
    /// Computes the translation for an origin. The resulting translation is applied to the mesh of the SVG.
    pub fn compute_translation(&self, size: Vec2) -> Vec3 {
        match self {
            Origin::BottomLeft => Vec3::new(0.0, size.y, 0.0),
            Origin::BottomRight => Vec3::new(-size.x, size.y, 0.0),
            Origin::Center => Vec3::new(-size.x * 0.5, size.y * 0.5, 0.0),
            // Standard SVG origin is top left, so we don't need to do anything
            Origin::TopLeft => Vec3::ZERO,
            Origin::TopRight => Vec3::new(-size.x, 0.0, 0.0),
            Origin::Custom(coord) => Vec3::new(-size.x * coord.0, size.y * coord.1, 0.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct OriginOffset([FloatOrd; 2]);

#[derive(Default, Resource)]
pub(crate) struct OriginMeshes(HashMap<(AssetId<Svg>, OriginOffset), Weak<StrongHandle>>);

impl OriginMeshes {
    pub(crate) fn remove_svg(&mut self, svg_id: AssetId<Svg>) {
        self.0.retain(|(id, _), _| *id != svg_id);
    }

    pub(crate) fn get_or_insert(
        &mut self,
        svg_id: AssetId<Svg>,
        svg: &Svg,
        origin: &Origin,
        meshes: &mut Assets<Mesh>,
    ) -> Handle<Mesh> {
        let translation = origin.compute_translation(svg.size);
        if translation == Vec3::ZERO {
            return svg.mesh.clone();
        }
        let key = (
            svg_id,
            OriginOffset(translation.truncate().to_array().map(FloatOrd)),
        );
        if let Some(handle) = self.0.get(&key).and_then(Weak::upgrade) {
            return Handle::Strong(handle);
        }
        self.0.retain(|_, handle| handle.strong_count() > 0);
        let Some(mesh) = meshes.get(&svg.mesh) else {
            return svg.mesh.clone();
        };
        let handle = meshes.add(mesh.clone().translated_by(translation));
        let Handle::Strong(strong) = &handle else {
            unreachable!("`Assets::add` returns a strong handle");
        };
        self.0.insert(key, Arc::downgrade(strong));
        handle
    }
}

/// Gets all SVGs with a changed origin and moves their mesh to the new origin.
pub fn apply_origin(
    svgs: Res<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut origin_meshes: ResMut<OriginMeshes>,
    mut query: Query<
        (
            Option<&Svg2d>,
            Option<&Svg3d>,
            &Origin,
            Option<&mut Mesh2d>,
            Option<&mut Mesh3d>,
        ),
        Changed<Origin>,
    >,
) {
    for (svg2d_handle, svg3d_handle, origin, mesh_2d, mesh_3d) in &mut query {
        let Some(svg_handle) =
            svg2d_handle.map_or_else(|| svg3d_handle.map(|x| &x.0), |x| Some(&x.0))
        else {
            continue;
        };
        let Some(svg) = svgs.get(svg_handle) else {
            continue;
        };
        let mesh = origin_meshes.get_or_insert(svg_handle.id(), svg, origin, &mut meshes);
        #[cfg(feature = "2d")]
        if let Some(mut mesh_2d) = mesh_2d {
            mesh_2d.0 = mesh.clone();
        }
        #[cfg(feature = "3d")]
        if let Some(mut mesh_3d) = mesh_3d {
            mesh_3d.0 = mesh;
        }
    }
}
