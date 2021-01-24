use lyon_svg::parser::{Color, Length};
use lyon_tessellation::{LineCap, LineJoin, StrokeOptions};
use super::LengthExt;


#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stroke {
    pub color: Option<Color>,
    pub opacity: Option<f32>,
    pub width: Option<Length>,
    pub start_cap: Option<LineCap>,
    pub end_cap: Option<LineCap>,
    pub line_join: Option<LineJoin>,
    pub miter_limit: Option<f32>,
}

impl Stroke {
    pub fn with_color(mut self, color: Color) -> Stroke {
        self.color = Some(color);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Stroke {
        self.opacity = Some(opacity);
        self
    }

    pub fn with_width(mut self, width: Length) -> Stroke {
        self.width = Some(width);
        self
    }

    pub fn with_line_caps(mut self, cap: LineCap) -> Stroke {
        self.start_cap = Some(cap);
        self
    }

    pub fn with_start_cap(mut self, cap: LineCap) -> Stroke {
        self.start_cap = Some(cap);
        self.end_cap = Some(cap);
        self
    }

    pub fn with_end_cap(mut self, cap: LineCap) -> Stroke {
        self.end_cap = Some(cap);
        self
    }

    pub fn with_line_join(mut self, line_join: LineJoin) -> Stroke {
        self.line_join = Some(line_join);
        self
    }

    pub fn with_miter_limit(mut self, miter_limit: f32) -> Stroke {
        self.miter_limit = Some(miter_limit);
        self
    }

    /// Non-None values of the other Stroke are used to override the values of the self Stroke.
    pub fn override_with(mut self, other: Stroke) -> Stroke {
        self.color = other.color.or(self.color);
        self.opacity = other.opacity.or(self.opacity);
        self.width = other.width.or(self.width);
        self.start_cap = other.start_cap.or(self.start_cap);
        self.end_cap = other.end_cap.or(self.end_cap);
        self.line_join = other.line_join.or(self.line_join);
        self.miter_limit = other.miter_limit.or(self.miter_limit);
        self
    }

    pub fn to_options(&self) ->  StrokeOptions {
        StrokeOptions::default()
            .with_start_cap(self.start_cap.unwrap_or(StrokeOptions::DEFAULT_LINE_CAP))
            .with_end_cap(self.end_cap.unwrap_or(StrokeOptions::DEFAULT_LINE_CAP))
            .with_line_join(self.line_join.unwrap_or(StrokeOptions::DEFAULT_LINE_JOIN))
            .with_line_width(self.width.map(|w| w.as_pixels()).unwrap_or(StrokeOptions::DEFAULT_LINE_WIDTH))
            .with_miter_limit(self.miter_limit.unwrap_or(StrokeOptions::DEFAULT_MITER_LIMIT))
    }
}
