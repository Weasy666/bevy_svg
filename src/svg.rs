use std::{io::Read, path::PathBuf};
use bevy::{math::{Mat4, Vec3}, prelude::{Color, Transform, Visible}};
use lyon_geom::euclid::default::Transform2D;
use lyon_svg::{parser::ViewBox, path::PathEvent};
use lyon_tessellation::math::Point;

use crate::{bundle::SvgBundle, Convert};

/// A loaded and deserialized SVG file.
#[derive(Debug)]
pub struct Svg {
    /// The name of the file.
    pub name: String,
    /// Width of the SVG.
    pub width: f64,
    /// Height of the SVG.
    pub height: f64,
    /// ViewBox of the SVG.
    pub view_box: ViewBox,
    /// Origin of the coordinate system and as such the origin for the Bevy position.
    pub origin: Origin,
    /// All paths that make up the SVG
    pub paths: Vec<PathDescriptor>,
}

impl Svg {
    pub(crate) fn from_tree(tree: usvg::Tree) -> Svg {
        let view_box = tree.svg_node().view_box;
        let size = tree.svg_node().size;
        let mut descriptors = Vec::new();

        for node in tree.root().descendants() {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                let t = p.transform;
                let abs_t = Transform::from_matrix(
                    Mat4::from_cols(
                        [t.a.abs() as f32, t.b as f32,       0.0, 0.0].into(),
                        [t.c as f32,       t.d.abs() as f32, 0.0, 0.0].into(),
                        [0.0,              0.0,              1.0, 0.0].into(),
                        [t.e as f32,       t.f as f32,       0.0, 1.0].into()
                    )
                );

                if let Some(ref fill) = p.fill {
                    let color = match fill.paint {
                        usvg::Paint::Color(c) =>
                            Color::rgba_u8(c.red, c.green, c.blue, fill.opacity.to_u8()),
                        _ => Color::default(),
                    };

                    descriptors.push(PathDescriptor {
                        segments: p.convert().collect(),
                        abs_transform: abs_t,
                        color,
                        draw_type: DrawType::Fill,
                    });
                }

                if let Some(ref stroke) = p.stroke {
                    let (color, draw_type) = stroke.convert();

                    descriptors.push(PathDescriptor {
                        segments: p.convert().collect(),
                        abs_transform: abs_t,
                        color,
                        draw_type,
                    });
                }
            }
        }

        Svg {
            name: Default::default(),
            width: size.width(),
            height: size.height(),
            view_box: ViewBox {
                x: view_box.rect.x(),
                y: view_box.rect.y(),
                w: view_box.rect.width(),
                h: view_box.rect.height(),
            },
            origin: Default::default(),
            paths: descriptors,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

enum Data<'a> {
    Bytes(&'a [u8]),
    File(PathBuf),
    Reader(Box<dyn std::io::Read>),
}

/// Builder for loading a SVG file and building a [`SvgBundle`].
pub struct SvgBuilder<'a> {
    name: String,
    data: Data<'a>,
    origin: Origin,
    translation: Vec3,
    scale: Vec3,
    is_visible: bool,
}

impl<'a> SvgBuilder<'a> {
    /// Create a [`SvgBuilder`] to load a SVG from a file.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> SvgBuilder<'a> {
        let path = PathBuf::from(path.as_ref());
        SvgBuilder {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            data: Data::File(path),
            origin: Origin::default(),
            translation: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            is_visible: true,
        }
    }

    /// Create a [`SvgBuilder`] from a reader.
    pub fn from_reader<R: 'static + std::io::Read>(reader: R, name: &str) -> SvgBuilder<'a> {
        SvgBuilder {
            name: name.to_string(),
            data: Data::Reader(Box::new(reader)),
            origin: Origin::default(),
            translation: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            is_visible: true,
        }
    }

    /// Create a [`SvgBuilder`] from bytes.
    pub fn from_bytes(bytes: &'a [u8], name: &str) -> SvgBuilder<'a> {
        SvgBuilder {
            name: name.to_string(),
            data: Data::Bytes(bytes),
            origin: Origin::default(),
            translation: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            is_visible: true,
        }
    }

    /// Change the origin of the SVG's coordinate system. The origin is also the
    /// Bevy origin.
    pub fn origin(mut self, origin: Origin) -> SvgBuilder<'a> {
        self.origin = origin;
        self
    }

    /// Position at which the [`SvgBundle`] will be spawned in Bevy. The origin
    /// of the SVG coordinate system will be at this position.
    pub fn position(mut self, translation: Vec3) ->  SvgBuilder<'a> {
        self.translation = translation;
        self
    }

    /// Value by which the SVG will be scaled, default is (1.0, 1.0).
    pub fn scale(mut self, scale: Vec3) ->  SvgBuilder<'a> {
        self.scale = scale;
        self
    }

    /// Value by which the SVG will be scaled, default is (1.0, 1.0).
    pub fn is_visible(mut self, visible: bool) ->  SvgBuilder<'a> {
        self.is_visible = visible;
        self
    }

    /// Load and finish the SVG content into a [`SvgBundle`], which then will be
    /// spawned by the [`SvgPlugin`].
    pub fn build<'s>(self) -> Result<SvgBundle, Box<dyn std::error::Error>> {
        let mut opt = usvg::Options::default();
        opt.fontdb.load_system_fonts();

        let mut svg_data = Vec::new();
        match self.data {
            Data::Bytes(bytes) => svg_data = bytes.to_vec(),
            Data::File(path) => {
                let mut file = std::fs::File::open(path)?;
                file.read_to_end(&mut svg_data)?;
            },
            Data::Reader(mut reader) => { reader.read_to_end(&mut svg_data)?; },
        }

        let svg_tree = usvg::Tree::from_data(&svg_data, &opt.to_ref())?;

        let translation = match self.origin {
            Origin::Center => self.translation + Vec3::new(
                -svg_tree.svg_node().size.width() as f32 * self.scale.x / 2.0,
                svg_tree.svg_node().size.height() as f32 * self.scale.y / 2.0,
                0.0
            ),
            Origin::TopLeft => self.translation,
        };

        let svg = Svg {
            name: self.name,
            origin: self.origin,
            ..Svg::from_tree(svg_tree)
        };

        Ok(SvgBundle {
            visible: Visible {
                is_visible: self.is_visible,
                is_transparent: true,
            },
            ..SvgBundle::new(svg).at_position(translation).with_scale(self.scale)
        })
    }
}

#[derive(Debug)]
pub struct PathDescriptor {
    pub segments: Vec<PathEvent>,
    pub abs_transform: Transform,
    pub color: Color,
    pub draw_type: DrawType,
}

#[derive(Debug)]
pub enum DrawType {
    Fill,
    Stroke(lyon_tessellation::StrokeOptions),
}

// Taken from https://github.com/nical/lyon/blob/74e6b137fea70d71d3b537babae22c6652f8843e/examples/wgpu_svg/src/main.rs
struct PathConvIter<'a> {
    iter: std::slice::Iter<'a, usvg::PathSegment>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
    scale: Transform2D<f32>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }
        let mut return_event = None;
        let next = self.iter.next();
        match next {
            Some(usvg::PathSegment::MoveTo { x, y }) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = (x, y).convert();
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    return_event = Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    });
                } else {
                    self.first = (x, y).convert();
                    return_event = Some(PathEvent::Begin { at: self.first });
                }
            }
            Some(usvg::PathSegment::LineTo { x, y }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = (x, y).convert();
                return_event = Some(PathEvent::Line {
                    from,
                    to: self.prev,
                });
            }
            Some(usvg::PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = (x, y).convert();
                return_event = Some(PathEvent::Cubic {
                    from,
                    ctrl1: (x1, y1).convert(),
                    ctrl2: (x2, y2).convert(),
                    to: self.prev,
                });
            }
            Some(usvg::PathSegment::ClosePath) => {
                self.needs_end = false;
                self.prev = self.first;
                return_event = Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                });
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    return_event = Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    });
                }
            }
        }

        return return_event.map(|event| event.transformed(&self.scale));
    }
}

impl Convert<Point> for (&f64, &f64) {
    fn convert(self) -> Point {
        Point::new((*self.0) as f32, (*self.1) as f32)
    }
}

impl<'a> Convert<PathConvIter<'a>> for &'a usvg::Path {
    fn convert(self) -> PathConvIter<'a> {
        PathConvIter {
            iter: self.data.iter(),
            first: Point::new(0.0, 0.0),
            prev: Point::new(0.0, 0.0),
            deferred: None,
            needs_end: false,
            // For some reason the local transform of some paths has negative scale values.
            // Here we correct to positive values.
            scale: lyon_geom::Transform::scale(
                if self.transform.a < 0.0 { -1.0 } else { 1.0 },
                if self.transform.d < 0.0 { -1.0 } else { 1.0 }
            )
        }
    }
}

impl Convert<(Color, DrawType)> for &usvg::Stroke {
    fn convert(self) -> (Color, DrawType) {
        let color = match self.paint {
            usvg::Paint::Color(c) =>
                Color::rgba_u8(c.red, c.green, c.blue, self.opacity.to_u8()),
            _ => Color::default(),
        };

        let linecap = match self.linecap {
            usvg::LineCap::Butt => lyon_tessellation::LineCap::Butt,
            usvg::LineCap::Square => lyon_tessellation::LineCap::Square,
            usvg::LineCap::Round => lyon_tessellation::LineCap::Round,
        };
        let linejoin = match self.linejoin {
            usvg::LineJoin::Miter => lyon_tessellation::LineJoin::Miter,
            usvg::LineJoin::Bevel => lyon_tessellation::LineJoin::Bevel,
            usvg::LineJoin::Round => lyon_tessellation::LineJoin::Round,
        };

        let opt = lyon_tessellation::StrokeOptions::tolerance(0.01)
            .with_line_width(self.width.value() as f32)
            .with_line_cap(linecap)
            .with_line_join(linejoin);

        (color, DrawType::Stroke(opt))
    }
}
