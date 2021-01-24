use lyon_svg::parser::Color;
use lyon_tessellation::{path::FillRule, FillOptions};


#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fill {
    pub color: Option<Color>,
    pub opacity: Option<f32>,
    pub rule: Option<FillRule>,
}

impl Fill {
    pub fn with_color(mut self, color: Color) -> Fill {
        self.color = Some(color);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Fill {
        self.opacity = Some(opacity);
        self
    }

    pub fn with_rule(mut self, rule: FillRule) -> Fill {
        self.rule = Some(rule);
        self
    }

    /// Non-None values of the other Stroke are used to override the values of the self Stroke.
    pub fn override_with(mut self, other: Fill) -> Fill {
        self.color = other.color.or(self.color);
        self.opacity = other.opacity.or(self.opacity);
        self.rule = other.rule.or(self.rule);
        self
    }

    pub fn to_options(&self) ->  FillOptions {
        FillOptions::default()
            .with_fill_rule(self.rule.unwrap_or(FillOptions::DEFAULT_FILL_RULE))
    }
}
