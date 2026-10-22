import bevy_render::color_operations::srgb_to_linear;

// Every row of the gradient texture describes one gradient:
//
// texel 0: (kind, spread method, stop count, _)
// texel 1: linear: (x1, y1, x2, y2), radial: (fx, fy, cx, cy)
// texel 2: radial: (fr, r, _, _)
// texel 3 + 2 * i: (offset, _, _, _) of stop i
// texel 4 + 2 * i: non-premultiplied sRGB color of stop i

const KIND_LINEAR: u32 = 0u;

const SPREAD_PAD: u32 = 0u;
const SPREAD_REFLECT: u32 = 1u;

const HEADER_TEXELS: i32 = 3;

/// Evaluates the gradient in `row` of `gradients` at `position`, which is given in the
/// coordinate system of the gradient.
///
/// Returns a non-premultiplied, linear color.
fn gradient_color(gradients: texture_2d<f32>, row: u32, position: vec2<f32>) -> vec4<f32> {
    let y = i32(row);
    let header = textureLoad(gradients, vec2(0, y), 0);
    let geometry = textureLoad(gradients, vec2(1, y), 0);

    var t: f32;
    if u32(header.x) == KIND_LINEAR {
        let start = geometry.xy;
        let direction = geometry.zw - start;
        t = dot(position - start, direction) / dot(direction, direction);
    } else {
        let radii = textureLoad(gradients, vec2(2, y), 0);
        let radial_t = two_point_conical_t(position, geometry.xy, radii.x, geometry.zw, radii.y);
        if radial_t.y == 0.0 {
            return vec4(0.0);
        }
        t = radial_t.x;
    }

    t = apply_spread_method(t, u32(header.y));
    let color = stop_color(gradients, y, i32(header.z), t);
    return vec4(srgb_to_linear(color.rgb), color.a);
}

/// Finds the largest `t` for which `position` lies on the circle that is interpolated between
/// the focal circle and the end circle, with a non-negative radius.
///
/// Returns `(t, 1.0)`, or `(_, 0.0)` if there is no such circle and the point is not painted.
fn two_point_conical_t(
    position: vec2<f32>,
    focal: vec2<f32>,
    focal_radius: f32,
    center: vec2<f32>,
    radius: f32,
) -> vec2<f32> {
    // Solves |p - f - t * (c - f)| = fr + t * (r - fr) for t.
    let cd = center - focal;
    let pd = position - focal;
    let dr = radius - focal_radius;

    let a = dot(cd, cd) - dr * dr;
    let b = dot(pd, cd) + focal_radius * dr;
    let c = dot(pd, pd) - focal_radius * focal_radius;

    if abs(a) < 1e-6 {
        if abs(b) < 1e-6 {
            return vec2(0.0, 0.0);
        }
        let t = c / (2.0 * b);
        return vec2(t, select(0.0, 1.0, focal_radius + t * dr >= 0.0));
    }

    let discriminant = b * b - a * c;
    if discriminant < 0.0 {
        return vec2(0.0, 0.0);
    }
    let root = sqrt(discriminant);
    let t0 = (b + root) / a;
    let t1 = (b - root) / a;
    let t_max = max(t0, t1);
    if focal_radius + t_max * dr >= 0.0 {
        return vec2(t_max, 1.0);
    }
    let t_min = min(t0, t1);
    return vec2(t_min, select(0.0, 1.0, focal_radius + t_min * dr >= 0.0));
}

fn apply_spread_method(t: f32, spread_method: u32) -> f32 {
    if spread_method == SPREAD_PAD {
        return clamp(t, 0.0, 1.0);
    }
    if spread_method == SPREAD_REFLECT {
        let m = t - 2.0 * floor(t * 0.5);
        return select(m, 2.0 - m, m > 1.0);
    }
    return fract(t);
}

fn stop_color(gradients: texture_2d<f32>, y: i32, stop_count: i32, t: f32) -> vec4<f32> {
    var previous_offset = textureLoad(gradients, vec2(HEADER_TEXELS, y), 0).x;
    var previous_color = textureLoad(gradients, vec2(HEADER_TEXELS + 1, y), 0);
    if t <= previous_offset {
        return previous_color;
    }
    for (var i = 1; i < stop_count; i++) {
        let offset = textureLoad(gradients, vec2(HEADER_TEXELS + 2 * i, y), 0).x;
        let color = textureLoad(gradients, vec2(HEADER_TEXELS + 2 * i + 1, y), 0);
        if t <= offset {
            let span = offset - previous_offset;
            let f = select(1.0, (t - previous_offset) / span, span > 0.0);
            return mix(previous_color, color, f);
        }
        previous_offset = offset;
        previous_color = color;
    }
    return previous_color;
}
