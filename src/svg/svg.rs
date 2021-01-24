use std::path::PathBuf;
use bevy::math::Vec3;
use lyon_svg::{parser::{Length, ViewBox}};
use lyon_tessellation::{
    path::{Path as PathDescriptor, FillRule},
    LineCap, LineJoin,
};
use quick_xml::de::from_str;
use serde::{de, Deserialize, Deserializer};
use crate::bundle::SvgBundle;

use super::{Fill, LengthExt, serde_utils, Stroke};

/// A loaded and deserialized SVG file.
#[derive(Debug, Deserialize)]
pub struct Svg {
    /// The name of the file.
    #[serde(skip)]
    pub file: String,
    /// Width of the SVG.
    #[serde(deserialize_with = "serde_utils::string_to_length")]
    pub width: Length,
    /// Height of the SVG.
    #[serde(deserialize_with = "serde_utils::string_to_length")]
    pub height: Length,
    /// ViewBox of the SVG.
    #[serde(rename = "viewBox", deserialize_with = "serde_utils::string_to_viewbox")]
    pub view_box: ViewBox,
    /// Origin of the coordinate system and as such the origin for the Bevy position.
    #[serde(skip)]
    pub origin: Origin,
    /// Global style of the SVG.
    pub style: Style,
    #[serde(rename = "path")]
    /// Paths present in the SVG.
    pub paths: Vec<Path>,
}

#[derive(Clone, Debug, PartialEq)]
/// Origin of the coordinate system.
pub enum Origin {
    /// Top left of the image or viewbox, this is the default for a SVG.
    TopLeft,
    /// Center of the image or viewbox.
    Center,
}

impl Default for Origin {
    fn default() -> Self {
        Origin::TopLeft
    }
}

/// Builder for loading a SVG file and building a [`SvgBundle`].
pub struct SvgBuilder {
    file: PathBuf,
    origin: Origin,
    translation: Vec3,
}

impl SvgBuilder {
    /// Create a [`SvgBuilder`] to load a SVG from a file.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> SvgBuilder {
        SvgBuilder {
            file: PathBuf::from(path.as_ref()),
            origin: Origin::default(),
            translation: Vec3::default(),
        }
    }

    /// Change the origin of the SVG's coordinate system. The origin is also the
    /// Bevy origin.
    pub fn origin(mut self, origin: Origin) -> SvgBuilder {
        self.origin = origin;
        self
    }

    /// Position at which the [`SvgBundle`] will be spawned in Bevy. The origin
    /// of the SVG coordinate system will be at this position.
    pub fn position(mut self, translation: Vec3) ->  SvgBuilder {
        self.translation = translation;
        self
    }

    /// Load and finish the SVG content into a [`SvgBundle`], which then will be
    /// spawned by the [`SvgPlugin`].
    pub fn build(self) -> Result<SvgBundle, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&self.file)?;
        let mut svg: Svg = from_str(&content)?;
        svg.file = self.file.file_name().unwrap().to_string_lossy().to_string();
        svg.origin = self.origin;

        Ok(SvgBundle::new(svg).at_position(self.translation))
    }
}


#[derive(Clone, Debug, Deserialize)]
pub struct Path {
    #[serde(deserialize_with = "serde_utils::string_to_path_description")]
    pub d: PathDescriptor,
    pub style: Style,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.style.eq(&other.style) && self.d.iter().eq(other.d.iter())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
/// Representation of the SVG style attribute.
pub struct Style {
    /// Fill options of a SVG shape.
    pub fill: Option<Fill>,
    /// Stroke options of a SVG shape.
    pub stroke: Option<Stroke>,
}

impl<'de> Deserialize<'de> for Style {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StyleVisitor;

        impl<'de> de::Visitor<'de> for StyleVisitor {
            type Value = Style;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a ';' separated string of SVG styles")
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                let style_strings = s.trim_matches(';')
                    .split(';')
                    .collect::<Vec<_>>();

                let mut style = Style::default();
                for style_string in style_strings {
                    let split_style = style_string.split(":").collect::<Vec<_>>();
                    if split_style.len() != 2 {
                        return Err(de::Error::invalid_value(de::Unexpected::Str(&format!("Found '{:?}'. This should either be an attribut with a value, or a value with an attribute.", split_style[0])), &self));
                    }

                    match split_style[0].to_lowercase().as_str() {
                        "fill" => {
                            let color = split_style[1].parse().map_err(|e| E::custom(format!("{}", e)))?;
                            style.fill = Some(style.fill.unwrap_or_default().with_color(color));
                        },
                        "fill-rule" => {
                            let fill_rule =  match split_style[1].to_lowercase().as_str() {
                                "evenodd" => FillRule::EvenOdd,
                                "nonzero" => FillRule::NonZero,
                                _  => Err(E::custom(format!("'{}' is not a valid FillRule", split_style[0])))?,
                            };
                            style.fill = Some(style.fill.unwrap_or_default().with_rule(fill_rule));
                        },
                        "fill-opacity" => {
                            let opacity = split_style[1].parse().map_err(|e| E::custom(format!("{}", e)))?;
                            style.fill = Some(style.fill.unwrap_or_default().with_opacity(opacity));
                        },
                        "stroke" => {
                            let color = split_style[1].parse().map_err(|e| E::custom(format!("{}", e)))?;
                            style.stroke = Some(style.stroke.unwrap_or_default().with_color(color));
                        },
                        "stroke-opacity" => {
                            let opacity = split_style[1].parse().map_err(|e| E::custom(format!("{}", e)))?;
                            style.stroke = Some(style.stroke.unwrap_or_default().with_opacity(opacity));
                        },
                        "stroke-linecap" => {
                            let line_caps = match split_style[1].to_lowercase().as_str() {
                                "butt"   => LineCap::Butt,
                                "square" => LineCap::Square,
                                "round"  => LineCap::Round,
                                _  => Err(E::custom(format!("'{}' is not a valid LineCap", split_style[0])))?,
                            };
                            style.stroke = Some(style.stroke.unwrap_or_default().with_line_caps(line_caps));
                        },
                        "stroke-linejoin" => {
                            let line_join = match split_style[1].to_lowercase().as_str() {
                                "miter"     => LineJoin::Miter,
                                "miterclip" => LineJoin::MiterClip,
                                "round"     => LineJoin::Round,
                                "bevel"     => LineJoin::Bevel,
                                _  => Err(E::custom(format!("'{}' is not a valid LineJoin", split_style[0])))?,
                            };
                            style.stroke = Some(style.stroke.unwrap_or_default().with_line_join(line_join));
                        },
                        "stroke-miterlimit" => {
                            let miter_limit = split_style[1].parse().map_err(|e| E::custom(format!("{}", e)))?;
                            style.stroke = Some(style.stroke.unwrap_or_default().with_miter_limit(miter_limit));
                        },
                        "stroke-width" => {
                            let width = split_style[1].parse().map_err(|e| E::custom(format!("{}", e)))?;
                            style.stroke = Some(style.stroke.unwrap_or_default().with_width(width));
                        },
                        "clip-rule" => (), // This is currently not supported, but it is part of some SVGs i use and so i don't want to error out of the deserialization
                        _ => Err(de::Error::invalid_value(de::Unexpected::Str(&format!("'{:?}' is a style attribute that is not supported by us.", split_style[0])), &self))?,
                    };
                }
                Ok(style)
            }
        }

        deserializer.deserialize_any(StyleVisitor)
    }
}


impl LengthExt for Length {
    fn as_pixels(self) -> f32 {
        //TODO: Need do implement a conversion from all possible LengthUnits into pixels.
        (match self.unit {
            //lyon_svg::parser::LengthUnit::None => ,
            //lyon_svg::parser::LengthUnit::Em => ,
            //lyon_svg::parser::LengthUnit::Ex => ,
            lyon_svg::parser::LengthUnit::Px => self.num,
            //lyon_svg::parser::LengthUnit::In => ,
            //lyon_svg::parser::LengthUnit::Cm => ,
            //lyon_svg::parser::LengthUnit::Mm => ,
            //lyon_svg::parser::LengthUnit::Pt => ,
            //lyon_svg::parser::LengthUnit::Pc => ,
            //lyon_svg::parser::LengthUnit::Percent => ,
            _ => todo!("{}", self.to_string()),
        }) as f32
    }
}
