use bevy::prelude::{Deref, DerefMut, Resource};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct FillTessellator(lyon_tessellation::FillTessellator);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct StrokeTessellator(lyon_tessellation::StrokeTessellator);
