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
mod plugin;
mod render;
mod svg;

/// Import this module as `use bevy_svg::prelude::*` to get convenient imports.
pub mod prelude {
    #[cfg(feature = "2d")]
    pub use crate::render::Svg2dBundle;
    #[cfg(feature = "3d")]
    pub use crate::render::Svg3dBundle;
    pub use crate::{
        plugin::SvgPlugin,
        svg::{Origin, Svg},
    };
    pub use lyon_tessellation::{
        FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };
}

/// A locally defined [`std::convert::Into`] surrogate to overcome orphan rules.
pub trait Convert<T>: Sized {
    /// Converts the value to `T`.
    fn convert(self) -> T;
}
