use lyon_geom::Transform;
use lyon_svg::{parser::{Length, Path as LyonPath, ViewBox}, path_utils};
use lyon_tessellation::path::Path as PathDescriptor;
use serde::{de, Deserializer};


pub fn string_to_length<'de, D>(deserializer: D) -> Result<Length, D::Error>
where
    D: Deserializer<'de>
{
    struct LengthVisitor;

    impl<'de> de::Visitor<'de> for LengthVisitor {
        type Value = Length;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a number with an optional unit, i.e. 1px.")
        }

        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
            s.parse()
                .map_err(|e| E::custom(format!("{}", e)))
        }
    }

    deserializer.deserialize_any(LengthVisitor)
}

pub fn string_to_path_description<'de, D>(deserializer: D) -> Result<PathDescriptor, D::Error>
where
    D: Deserializer<'de>
{
    struct PathDescriptorVisitor;

    impl<'de> de::Visitor<'de> for PathDescriptorVisitor {
        type Value = PathDescriptor;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a comma or space separated string of numbers and path commands")
        }

        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
            let mut absolute_path = s.parse::<LyonPath>()
                .map_err(|e| E::custom(format!("{}", e)))?;
            absolute_path.conv_to_absolute();

            let path_builder = PathDescriptor::builder()
                .with_svg()
                .transformed(Transform::scale(1.0, -1.0));
            path_utils::build_path(path_builder, &absolute_path.to_string())
                .map_err(|e| E::custom(format!("{:?}", e)))
        }
    }

    deserializer.deserialize_any(PathDescriptorVisitor)
}

pub fn string_to_viewbox<'de, D>(deserializer: D) -> Result<ViewBox, D::Error>
where
    D: Deserializer<'de>
{
    struct ViewBoxVisitor;

    impl<'de> de::Visitor<'de> for ViewBoxVisitor {
        type Value = ViewBox;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a comma or space separated string of integer numbers")
        }

        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
            s.parse()
                .map_err(|e| E::custom(format!("{}", e)))
        }
    }

    deserializer.deserialize_any(ViewBoxVisitor)
}
