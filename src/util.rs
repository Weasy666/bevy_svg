pub mod paint {
    use bevy::color::{Color, ColorToComponents, Srgba};
    use usvg::BaseGradient;

    use crate::Convert;

    trait ToF32Array {
        fn to_f32_array(&self) -> [f32; 4];
    }

    impl ToF32Array for Option<&usvg::Stop> {
        fn to_f32_array(&self) -> [f32; 4] {
            self.map(Convert::convert)
                .unwrap_or(Color::NONE)
                .to_srgba()
                .to_f32_array()
        }
    }

    pub fn avg_gradient(gradient: &BaseGradient) -> Color {
        let first = gradient.stops().first().to_f32_array();
        let last = gradient.stops().last().to_f32_array();
        let avg = [
            first[0] + last[0],
            first[1] + last[1],
            first[2] + last[2],
            first[3] + last[3],
        ]
        .map(|x| x / 2.0);
        Color::Srgba(Srgba::from_f32_array(avg))
    }
}
