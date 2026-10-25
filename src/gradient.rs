use bevy::{
    asset::RenderAssetUsages,
    color::{Alpha, Color, ColorToComponents, Srgba},
    image::Image,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use std::collections::HashMap;

/// How a tessellated path is painted.
#[derive(Debug, Clone, Copy)]
pub enum PathPaint {
    Color(Color),
    Gradient {
        /// Row of the gradient in the gradient texture.
        index: u32,
        opacity: f32,
        /// Maps the path's local coordinates into the gradient's coordinate system.
        local_to_gradient: usvg::Transform,
    },
}

impl PathPaint {
    pub(crate) fn from_usvg(
        paint: &usvg::Paint,
        opacity: usvg::Opacity,
        gradients: &mut Gradients,
    ) -> Self {
        let color = |c: usvg::Color, alpha: usvg::Opacity| {
            PathPaint::Color(Color::srgba_u8(c.red, c.green, c.blue, alpha.to_u8()))
        };
        let gradient = match paint {
            usvg::Paint::Color(c) => return color(*c, opacity),
            usvg::Paint::LinearGradient(g) => {
                let (start, end) = ((g.x1(), g.y1()), (g.x2(), g.y2()));
                if start == end {
                    // https://www.w3.org/TR/SVG11/pservers.html#LinearGradientElementX2Attribute
                    let Some(last) = g.stops().last() else {
                        return PathPaint::Color(Color::NONE);
                    };
                    return color(last.color(), last.opacity() * opacity);
                }
                GradientRef {
                    base: g,
                    geometry: GradientGeometry::Linear { start, end },
                }
            }
            // usvg already turns `r <= 0` into the color of the last stop
            usvg::Paint::RadialGradient(g) => GradientRef {
                base: g,
                geometry: GradientGeometry::Radial {
                    focal: (g.fx(), g.fy()),
                    focal_radius: g.fr().get(),
                    center: (g.cx(), g.cy()),
                    radius: g.r().get(),
                },
            },
            usvg::Paint::Pattern(_) => return PathPaint::Color(Color::NONE),
        };
        let Some(local_to_gradient) = gradient.base.transform().invert() else {
            return PathPaint::Color(Color::NONE);
        };
        PathPaint::Gradient {
            index: gradients.insert(gradient),
            opacity: opacity.get(),
            local_to_gradient,
        }
    }

    /// Whether every pixel of the path is painted with full opacity.
    pub(crate) fn is_opaque(&self, gradients: &[Gradient]) -> bool {
        match *self {
            PathPaint::Color(color) => color.is_fully_opaque(),
            PathPaint::Gradient { index, opacity, .. } => {
                opacity >= 1.0 && gradients[index as usize].is_opaque()
            }
        }
    }
}

struct GradientRef<'a> {
    base: &'a usvg::BaseGradient,
    geometry: GradientGeometry,
}

#[derive(Debug, Clone, Copy)]
enum GradientGeometry {
    Linear {
        start: (f32, f32),
        end: (f32, f32),
    },
    Radial {
        focal: (f32, f32),
        focal_radius: f32,
        center: (f32, f32),
        radius: f32,
    },
}

/// A gradient as it is uploaded to the GPU, see `gradient.wesl` for the layout.
#[derive(Debug, Clone)]
pub struct Gradient {
    geometry: GradientGeometry,
    spread_method: usvg::SpreadMethod,
    /// Offset and color of every stop.
    stops: Vec<(f32, Srgba)>,
}

impl Gradient {
    fn is_opaque(&self) -> bool {
        let covers_plane = match self.geometry {
            GradientGeometry::Linear { .. } => true,
            GradientGeometry::Radial {
                focal,
                focal_radius,
                center,
                radius,
            } => {
                // Unless one circle lies inside the other, we leave some points unpainted.
                let center_offset = (center.0 - focal.0, center.1 - focal.1);
                let radius_delta = radius - focal_radius;
                let a = center_offset.0 * center_offset.0 + center_offset.1 * center_offset.1
                    - radius_delta * radius_delta;
                a < -1e-6 // threshold as in the shader
            }
        };
        covers_plane && self.stops.iter().all(|(_, color)| color.is_fully_opaque())
    }
}

/// Collects the gradients used by an SVG, deduplicating the ones shared between paths.
#[derive(Default)]
pub(crate) struct Gradients {
    gradients: Vec<Gradient>,
    indices: HashMap<*const usvg::BaseGradient, u32>,
}

impl Gradients {
    fn insert(&mut self, gradient: GradientRef) -> u32 {
        *self
            .indices
            .entry(std::ptr::from_ref(gradient.base))
            .or_insert_with(|| {
                self.gradients.push(Gradient {
                    geometry: gradient.geometry,
                    spread_method: gradient.base.spread_method(),
                    stops: gradient
                        .base
                        .stops()
                        .iter()
                        .map(|stop| {
                            let c = stop.color();
                            let color = Srgba::rgb_u8(c.red, c.green, c.blue)
                                .with_alpha(stop.opacity().get());
                            (stop.offset().get(), color)
                        })
                        .collect(),
                });
                self.gradients.len() as u32 - 1
            })
    }

    pub(crate) fn into_vec(self) -> Vec<Gradient> {
        self.gradients
    }
}

const HEADER_TEXELS: usize = 3;
const TEXELS_PER_STOP: usize = 2;

/// Packs `gradients` into an `Rgba32Float` texture with one row per gradient.
///
/// Returns `None` if there are no gradients.
pub(crate) fn gradient_image(gradients: &[Gradient]) -> Option<Image> {
    let max_stops = gradients.iter().map(|g| g.stops.len()).max()?;
    let width = HEADER_TEXELS + TEXELS_PER_STOP * max_stops;

    let mut texels = vec![[0.0f32; 4]; width * gradients.len()];
    for (row, gradient) in texels.chunks_exact_mut(width).zip(gradients) {
        let spread_method = match gradient.spread_method {
            usvg::SpreadMethod::Pad => 0.0,
            usvg::SpreadMethod::Reflect => 1.0,
            usvg::SpreadMethod::Repeat => 2.0,
        };
        let stop_count = gradient.stops.len() as f32;
        match gradient.geometry {
            GradientGeometry::Linear { start, end } => {
                row[0] = [0.0, spread_method, stop_count, 0.0];
                row[1] = [start.0, start.1, end.0, end.1];
            }
            GradientGeometry::Radial {
                focal,
                focal_radius,
                center,
                radius,
            } => {
                row[0] = [1.0, spread_method, stop_count, 0.0];
                row[1] = [focal.0, focal.1, center.0, center.1];
                row[2] = [focal_radius, radius, 0.0, 0.0];
            }
        }
        let (stop_texels, _) = row[HEADER_TEXELS..].as_chunks_mut::<TEXELS_PER_STOP>();
        for ([offset_texel, color_texel], (offset, color)) in
            stop_texels.iter_mut().zip(&gradient.stops)
        {
            *offset_texel = [*offset, 0.0, 0.0, 0.0];
            *color_texel = color.to_f32_array();
        }
    }

    Some(Image::new(
        Extent3d {
            width: width as u32,
            height: gradients.len() as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texels
            .into_iter()
            .flatten()
            .flat_map(f32::to_ne_bytes)
            .collect(),
        TextureFormat::Rgba32Float,
        RenderAssetUsages::default(),
    ))
}
