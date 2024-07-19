use std::collections::VecDeque;
use std::ops::Deref;
use std::path::PathBuf;

use bevy::{
    asset::{Asset, Handle},
    color::Color,
    log::{debug, trace, warn},
    math::Vec2,
    reflect::{std_traits::ReflectDefault, Reflect},
    render::{mesh::Mesh, render_resource::AsBindGroup},
};
use copyless::VecHelper;
use lyon_path::PathEvent;
use lyon_tessellation::{math::Point, FillTessellator, StrokeTessellator};
use svgtypes::ViewBox;
use usvg::{
    tiny_skia_path::{PathSegment, PathSegmentsIter},
    Node, PaintOrder,
};

use crate::{loader::FileSvgError, render::tessellation, Convert};

/// A loaded and deserialized SVG file.
#[derive(AsBindGroup, Reflect, Debug, Clone, Asset)]
#[reflect(Default, Debug)]
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
        path: impl Into<PathBuf> + Copy,
        fonts: Option<impl Into<PathBuf>>,
    ) -> Result<Svg, FileSvgError> {
        let svg_tree = usvg::Tree::from_data(&bytes, &usvg::Options::default()).map_err(|err| {
            FileSvgError {
                error: err.into(),
                path: format!("{}", path.into().display()),
            }
        })?;

        let mut fontdb = usvg::fontdb::Database::default();
        fontdb.load_system_fonts();
        let font_dir = fonts.map(|p| p.into()).unwrap_or("./assets".into());
        debug!("loading fonts in {:?}", font_dir);
        fontdb.load_fonts_dir(font_dir);

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
        let view_box = tree.root().layer_bounding_box();
        let size = tree.size();
        let mut descriptors = Vec::new();

        let mut node_stack = tree.root()
            .children()
            .into_iter()
            // to make sure we are processing the svg with sibling > descendant priority we reverse it
            // and reverse the resulting descriptors before returning the final constructed svg
            .rev()
            .collect::<VecDeque<_>>();

        while let Some(node) = node_stack.pop_front() {
            trace!("---");
            trace!("node: {}", node.id());
            match node {
                usvg::Node::Group(ref group) => {
                    trace!("group: {}", group.id());
                    if !group.should_isolate() {
                        for node in group.children() {
                            node_stack.push_front(node);
                        }
                    } else {
                        todo!("group isolate not implemented")
                    }
                }
                usvg::Node::Text(ref text) => {
                    trace!("text: {}", text.id());
                    let group = text.flattened();
                    for node in group.children() {
                        node_stack.push_front(node);
                    }
                }
                usvg::Node::Path(ref path) => {
                    if !path.is_visible() {
                        trace!("path: {} - invisible", path.id());
                        continue;
                    }
                    trace!("path: {}", path.id());
                    let transform = path.abs_transform();

                    let path_with_transform = PathWithTransform { path, transform };

                    match path.paint_order() {
                        PaintOrder::FillAndStroke => {
                            Self::process_fill(&mut descriptors, path_with_transform);
                            Self::process_stroke(&mut descriptors, path_with_transform);
                        }
                        PaintOrder::StrokeAndFill => {
                            Self::process_stroke(&mut descriptors, path_with_transform);
                            Self::process_fill(&mut descriptors, path_with_transform);
                        }
                    }
                }
                usvg::Node::Image(image) => {
                    warn!("image: {} - not implemented", image.id());
                }
            }
        }
        
        descriptors.reverse();

        Svg {
            name: Default::default(),
            size: Vec2::new(size.width(), size.height()),
            view_box: ViewBox {
                x: view_box.x() as f64,
                y: view_box.y() as f64,
                w: view_box.width() as f64,
                h: view_box.height() as f64,
            },
            paths: descriptors,
            mesh: Default::default(),
        }
    }

    fn process_fill(descriptors: &mut Vec<PathDescriptor>, path_with_transform: PathWithTransform) {
        let path = path_with_transform.path;
        // from resvg render logic
        if path.data().bounds().width() == 0.0 || path.data().bounds().height() == 0.0 {
            // Horizontal and vertical lines cannot be filled. Skip.
            return;
        }
        let Some(fill) = &path.fill() else {
            return;
        };
        let color = match fill.paint() {
            usvg::Paint::Color(c) => {
                Color::srgba_u8(c.red, c.green, c.blue, fill.opacity().to_u8())
            }
            usvg::Paint::LinearGradient(g) => {
                // TODO: implement
                // just taking the average between the first and last stop so we get something to render
                crate::util::paint::avg_gradient(g.deref().deref())
            }
            usvg::Paint::RadialGradient(g) => {
                // TODO: implement
                // just taking the average between the first and last stop so we get something to render
                crate::util::paint::avg_gradient(g.deref().deref())
            }
            _ => Color::NONE,
        };

        descriptors.alloc().init(PathDescriptor {
            segments: path_with_transform.convert().collect(),
            color,
            draw_type: DrawType::Fill,
        });
    }

    fn process_stroke(
        descriptors: &mut Vec<PathDescriptor>,
        path_with_transform: PathWithTransform,
    ) {
        let path = path_with_transform.path;
        let Some(stroke) = &path.stroke() else { return };
        let (color, draw_type) = stroke.convert();

        descriptors.alloc().init(PathDescriptor {
            segments: path_with_transform.convert().collect(),
            color,
            draw_type,
        });
    }
}

#[derive(Debug, Clone)]
pub struct PathDescriptor {
    pub segments: Vec<PathEvent>,
    pub color: Color,
    pub draw_type: DrawType,
}

#[derive(Debug, Clone)]
pub enum DrawType {
    Fill,
    Stroke(lyon_tessellation::StrokeOptions),
}

#[derive(Debug, Copy, Clone)]
struct PathWithTransform<'a> {
    path: &'a usvg::Path,
    transform: usvg::Transform,
}

// Taken from https://github.com/nical/lyon/blob/74e6b137fea70d71d3b537babae22c6652f8843e/examples/wgpu_svg/src/main.rs
pub(crate) struct PathConvIter<'iter> {
    iter: PathSegmentsIter<'iter>,
    transform: usvg::Transform,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
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

        return_event.map(|event| {
            // Mapping for converting usvg::Transform to lyon_geom::Transform
            //
            // | sx  kx  tx | -> | m11 m21 m31 |
            // | ky  sy  ty | -> | m12 m22 m32 |
            // |  0   0   1 | -> |   0   0   1 |
            //
            // m11, m12,
            // m21, m22,
            // m31, m32,
            event.transformed(&lyon_geom::Transform::new(
                self.transform.sx,
                self.transform.ky,
                self.transform.kx,
                self.transform.sy,
                self.transform.tx,
                self.transform.ty,
            ))
        })
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

impl<'iter> Convert<PathConvIter<'iter>> for PathWithTransform<'iter> {
    fn convert(self) -> PathConvIter<'iter> {
        return PathConvIter {
            iter: self.path.data().segments(),
            transform: self.transform,
            first: Point::new(0.0, 0.0),
            prev: Point::new(0.0, 0.0),
            deferred: None,
            needs_end: false,
        };
    }
}

impl Convert<(Color, DrawType)> for &usvg::Stroke {
    #[inline]
    fn convert(self) -> (Color, DrawType) {
        let color = match self.paint() {
            usvg::Paint::Color(c) => {
                Color::srgba_u8(c.red, c.green, c.blue, self.opacity().to_u8())
            }
            // TODO: implement, take average for now
            usvg::Paint::LinearGradient(g) => crate::util::paint::avg_gradient(g.deref().deref()),
            usvg::Paint::RadialGradient(g) => crate::util::paint::avg_gradient(g.deref().deref()),
            usvg::Paint::Pattern(_) => Color::NONE,
        };

        let linecap = match self.linecap() {
            usvg::LineCap::Butt => lyon_tessellation::LineCap::Butt,
            usvg::LineCap::Square => lyon_tessellation::LineCap::Square,
            usvg::LineCap::Round => lyon_tessellation::LineCap::Round,
        };
        let linejoin = match self.linejoin() {
            usvg::LineJoin::Miter => lyon_tessellation::LineJoin::Miter,
            usvg::LineJoin::MiterClip => lyon_tessellation::LineJoin::MiterClip,
            usvg::LineJoin::Bevel => lyon_tessellation::LineJoin::Bevel,
            usvg::LineJoin::Round => lyon_tessellation::LineJoin::Round,
        };

        let opt = lyon_tessellation::StrokeOptions::tolerance(0.01)
            .with_line_width(self.width().get())
            .with_line_cap(linecap)
            .with_line_join(linejoin);

        return (color, DrawType::Stroke(opt));
    }
}
