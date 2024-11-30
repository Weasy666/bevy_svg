mod plugin;
pub mod tessellation;
mod vertex_buffer;

#[cfg(feature = "2d")]
mod svg2d;
#[cfg(feature = "3d")]
mod svg3d;

#[cfg(feature = "2d")]
pub use svg2d::{Svg2d, Svg2dBundle};
#[cfg(feature = "3d")]
pub use svg3d::{Svg3d, Svg3dBundle};

pub use plugin::SvgPlugin;
