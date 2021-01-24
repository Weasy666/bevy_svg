//! Representation of a simple SVG. Simple because a lot of stuff is missing.

mod fill;
pub(crate) mod serde_utils;
mod stroke;
mod svg;


use fill::Fill;
use stroke::Stroke;
pub use svg::{Origin, SvgBuilder, Svg, Style};


/// An trait to easily extend the functions of the [`svgtypes::Length`] type.
pub trait LengthExt {
    /// The length in pixels.
    fn as_pixels(self) -> f32;
}
