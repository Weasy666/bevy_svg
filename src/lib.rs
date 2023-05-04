//! Load and disply simple SVG files in Bevy.
//!
//! This crate provides a Bevy `Plugin` to easily load and display a simple SVG file.
//!
//! ## Usage
//! Simply add the crate in your `Cargo.toml` and add the plugin to your app
//!
//! ```rust
//! fn main() {
//!     App::new()
//!         .add_plugin(bevy_svg::prelude::SvgPlugin)
//!         .run();
//! }
//! ```

// rustc
#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, unused)]
#![allow(elided_lifetimes_in_paths)]
// clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod loader;
#[cfg(any(feature = "2d", feature = "3d"))]
mod origin;
#[cfg(any(feature = "2d", feature = "3d"))]
mod plugin;
mod render;
mod resources;
mod svg;

/// Import this module as `use bevy_svg::prelude::*` to get convenient imports.
pub mod prelude {
    #[cfg(any(feature = "2d", feature = "3d"))]
    pub use crate::origin::Origin;
    #[cfg(any(feature = "2d", feature = "3d"))]
    use crate::plugin::SvgRenderPlugin;
    #[cfg(feature = "2d")]
    pub use crate::render::Svg2dBundle;
    #[cfg(feature = "3d")]
    pub use crate::render::Svg3dBundle;
    pub use crate::svg::Svg;
    pub use lyon_tessellation::{
        FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };

    use crate::loader::SvgAssetLoader;
    use bevy::{
        app::{App, Plugin},
        asset::AddAsset,
    };

    /// A plugin that provides resources and a system to draw [`Svg`]s.
    pub struct SvgPlugin;

    impl Plugin for SvgPlugin {
        fn build(&self, app: &mut App) {
            app.add_asset::<Svg>().init_asset_loader::<SvgAssetLoader>();
            #[cfg(any(feature = "2d", feature = "3d"))]
            app.add_plugin(SvgRenderPlugin);
        }
    }
}

/// A locally defined [`std::convert::Into`] surrogate to overcome orphan rules.
pub trait Convert<T>: Sized {
    /// Converts the value to `T`.
    fn convert(self) -> T;
}
