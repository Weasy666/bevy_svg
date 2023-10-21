use std::path::PathBuf;

use bevy::{
    asset::{Asset, Handle},
    math::{Mat4, Vec2},
    reflect::{std_traits::ReflectDefault, Reflect, TypeUuid},
    render::{color::Color, mesh::Mesh, render_resource::AsBindGroup},
    transform::components::Transform,
};
use copyless::VecHelper;
use lyon_geom::euclid::default::Transform2D;
use lyon_path::PathEvent;
use lyon_tessellation::{math::Point, FillTessellator, StrokeTessellator};
use svgtypes::ViewBox;
use usvg::{
    tiny_skia_path::{PathSegment, PathSegmentsIter},
    NodeExt, TreeParsing, TreeTextToPath,
};

use crate::{loader::FileSvgError, render::tessellation, Convert};

/// A loaded and deserialized SVG file.
#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid, Asset)]
#[reflect(Default, Debug)]
#[uuid = "ad47a360-355d-4955-9fd8-678412a77f12"]
pub struct Svg {
    /// The name of the file.
    pub name: String,
    /// Size of the SVG.
    pub size: Vec2,
    #[reflect(ignore)]
    /// ViewBox of the SVG.
    pub view_box: ViewBox,
    #[reflect(ignore)]
    /// All paths that make up the SVG.
    pub paths: Vec<PathDescriptor>,
    /// The fully tessellated paths as [`Mesh`].
    pub mesh: Handle<Mesh>,
}

impl Default for Svg {
    fn default() -> Self {
        Self {
            name: Default::default(),
            size: Default::default(),
            view_box: ViewBox {
                x: 0.,
                y: 0.,
                w: 0.,
                h: 0.,
            },
            paths: Default::default(),
            mesh: Default::default(),
        }
    }
}

impl Svg {
    /// Loads an SVG from bytes
    pub fn from_bytes(
        bytes: &[u8],
        path: impl Into<PathBuf>,
        fonts: Option<impl Into<PathBuf>>,
    ) -> Result<Svg, FileSvgError> {
        let mut svg_tree =
            usvg::Tree::from_data(&bytes, &usvg::Options::default()).map_err(|err| {
                FileSvgError {
                    error: err.into(),
                    path: format!("{}", path.into().display()),
                }
            })?;

        let mut fontdb = usvg::fontdb::Database::default();
        fontdb.load_system_fonts();
        fontdb.load_fonts_dir(fonts.map(|p| p.into()).unwrap_or("./assets".into()));
        svg_tree.convert_text(&fontdb);

        Ok(Svg::from_tree(svg_tree))
    }

    /// Creates a bevy mesh from the SVG data.
    pub fn tessellate(&self) -> Mesh {
        let buffer = tessellation::generate_buffer(
            self,
            &mut FillTessellator::new(),
            &mut StrokeTessellator::new(),
        );
        buffer.convert()
    }

    pub(crate) fn from_tree(tree: usvg::Tree) -> Svg {
        let view_box = tree.view_box;
        let size = tree.size;
        let mut descriptors = Vec::new();

        for node in tree.root.descendants() {
            match &*node.borrow() {
                usvg::NodeKind::Path(ref path) => {
                    let abs_transform = node.abs_transform().convert();

                    if let Some(fill) = &path.fill {
                        let color = match fill.paint {
                            usvg::Paint::Color(c) => {
                                Color::rgba_u8(c.red, c.green, c.blue, fill.opacity.to_u8())
                            }
                            _ => Color::default(),
                        };

                        descriptors.alloc().init(PathDescriptor {
                            segments: path.convert().collect(),
                            abs_transform,
                            color,
                            draw_type: DrawType::Fill,
                        });
                    }

                    if let Some(stroke) = &path.stroke {
                        let (color, draw_type) = stroke.convert();

                        descriptors.alloc().init(PathDescriptor {
                            segments: path.convert().collect(),
                            abs_transform,
                            color,
                            draw_type,
                        });
                    }
                }
                _ => {}
            }
        }

        return Svg {
            name: Default::default(),
            size: Vec2::new(size.width(), size.height()),
            view_box: ViewBox {
                x: view_box.rect.x() as f64,
                y: view_box.rect.y() as f64,
                w: view_box.rect.width() as f64,
                h: view_box.rect.height() as f64,
            },
            paths: descriptors,
            mesh: Default::default(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct PathDescriptor {
    pub segments: Vec<PathEvent>,
    pub abs_transform: Transform,
    pub color: Color,
    pub draw_type: DrawType,
}

#[derive(Debug, Clone)]
pub enum DrawType {
    Fill,
    Stroke(lyon_tessellation::StrokeOptions),
}

// Taken from https://github.com/nical/lyon/blob/74e6b137fea70d71d3b537babae22c6652f8843e/examples/wgpu_svg/src/main.rs
pub(crate) struct PathConvIter<'iter> {
    iter: PathSegmentsIter<'iter>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
    scale: Transform2D<f32>,
}

impl<'iter> Iterator for PathConvIter<'iter> {
    type Item = PathEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }
        let mut return_event = None;
        let next = self.iter.next();
        match next {
            Some(PathSegment::MoveTo(point)) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = point.convert();
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    return_event = Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    });
                } else {
                    self.first = point.convert();
                    return_event = Some(PathEvent::Begin { at: self.first });
                }
            }
            Some(PathSegment::LineTo(point)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point.convert();
                return_event = Some(PathEvent::Line {
                    from,
                    to: self.prev,
                });
            }
            Some(PathSegment::CubicTo(point1, point2, point3)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point3.convert();
                return_event = Some(PathEvent::Cubic {
                    from,
                    ctrl1: point1.convert(),
                    ctrl2: point2.convert(),
                    to: self.prev,
                });
            }
            Some(PathSegment::QuadTo(point1, point2)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point2.convert();
                return_event = Some(PathEvent::Quadratic {
                    from,
                    ctrl: point1.convert(),
                    to: self.prev,
                });
            }
            Some(PathSegment::Close) => {
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

impl Convert<Point> for &usvg::tiny_skia_path::Point {
    #[inline]
    fn convert(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl Convert<Point> for usvg::tiny_skia_path::Point {
    #[inline]
    fn convert(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl Convert<Transform> for usvg::tiny_skia_path::Transform {
    #[inline]
    fn convert(self) -> Transform {
        Transform::from_matrix(Mat4::from_cols(
            [self.sx, self.ky, 0.0, 0.0].into(),
            [self.kx, self.sy, 0.0, 0.0].into(),
            [0.0, 0.0, 1.0, 0.0].into(),
            [self.tx, self.ty, 0.0, 1.0].into(),
        ))
    }
}

impl<'iter> Convert<PathConvIter<'iter>> for &'iter usvg::Path {
    fn convert(self) -> PathConvIter<'iter> {
        let (scale_x, scale_y) = self.transform.get_scale();
        return PathConvIter {
            iter: self.data.segments(),
            first: Point::new(0.0, 0.0),
            prev: Point::new(0.0, 0.0),
            deferred: None,
            needs_end: false,
            scale: lyon_geom::Transform::scale(scale_x, scale_y),
        };
    }
}

impl Convert<(Color, DrawType)> for &usvg::Stroke {
    #[inline]
    fn convert(self) -> (Color, DrawType) {
        let color = match self.paint {
            usvg::Paint::Color(c) => Color::rgba_u8(c.red, c.green, c.blue, self.opacity.to_u8()),
            usvg::Paint::LinearGradient(_)
            | usvg::Paint::RadialGradient(_)
            | usvg::Paint::Pattern(_) => Color::default(),
        };

        let linecap = match self.linecap {
            usvg::LineCap::Butt => lyon_tessellation::LineCap::Butt,
            usvg::LineCap::Square => lyon_tessellation::LineCap::Square,
            usvg::LineCap::Round => lyon_tessellation::LineCap::Round,
        };
        let linejoin = match self.linejoin {
            usvg::LineJoin::Miter => lyon_tessellation::LineJoin::Miter,
            usvg::LineJoin::MiterClip => lyon_tessellation::LineJoin::MiterClip,
            usvg::LineJoin::Bevel => lyon_tessellation::LineJoin::Bevel,
            usvg::LineJoin::Round => lyon_tessellation::LineJoin::Round,
        };

        let opt = lyon_tessellation::StrokeOptions::tolerance(0.01)
            .with_line_width(self.width.get() as f32)
            .with_line_cap(linecap)
            .with_line_join(linejoin);

        return (color, DrawType::Stroke(opt));
    }
}
