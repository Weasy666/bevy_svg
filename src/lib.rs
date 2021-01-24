//! Load and disply simple SVG files in Bevy.
//!
//! This crate provides a Bevy `Plugin` to easily load and display a simple SVG file.
//! It currently only works for the most simple SVGs.
//!
//! ## Usage
//! Simply add the crate in your `Cargo.toml` and add the plugin to your app
//!
//! ```rust
//! fn main() {
//!     App::build()
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

pub mod bundle;
pub mod plugin;
pub mod svg;
pub mod utils;

/// Import this module as `use bevy_svg::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{bundle::SvgBundle, plugin::SvgPlugin, svg::{SvgBuilder, Origin}};
    pub use lyon_tessellation::{
        FillOptions, FillRule, LineCap, LineJoin, Orientation, StrokeOptions,
    };
}
