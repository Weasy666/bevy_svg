# bevy_svg
[![Crates.io](https://img.shields.io/crates/v/bevy_svg.svg)](https://crates.io/crates/bevy_svg)
[![license](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE)

For one of my personal projects i needed a way to load and display some simple SVG files/shapes in [`Bevy`],
so i took inspiration from [`bevy_prototype_lyon`] and modified and extended it to...well...load and display
simple SVG files. SVGs can be used/displayed in `2D` as well as in `3D`.

Files are loaded through [`AssetLoader`], then parsed and simplified with [`usvg`] and then tessellated with [`Lyon`]
into a vertex buffer, which lastly is convert into a [`Bevy`] mesh and drawn with custom shaders.

> *Note:* The SVG support is currently rather basic, i'd like to expand that in the future.


## Compatibility
| `Bevy` version | `bevy_svg` version | Branch      |
|----------------|--------------------|-------------|
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.16.0-rc1-orange)](https://crates.io/crates/bevy/0.16.0-rc1) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.16.0-rc1-orange)](https://crates.io/crates/bevy-svg/0.16.0-rc1) | [`bevy-0.16`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.16) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.15.1-orange)](https://crates.io/crates/bevy/0.15.1) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.15.1-orange)](https://crates.io/crates/bevy-svg/0.15.1) | [`bevy-0.15`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.15) |
| [![Crates.io](https://img.shields.io/badge/branch-main-yellow)](https://github.com/bevyengine/bevy) | [![Crates.io](https://img.shields.io/badge/branch-main-yellow)](https://github.com/Weasy666/bevy_svg/) | [`main`](https://github.com/Weasy666/bevy_svg) |

<details><summary>Old versions</summary>

| `Bevy` version | `bevy_svg` version | Branch      |
|----------------|--------------------|-------------|
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.14.0-orange)](https://crates.io/crates/bevy/0.14.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.14.0-orange)](https://crates.io/crates/bevy-svg/0.14.0) | [`bevy-0.14`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.14) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.12.0-orange)](https://crates.io/crates/bevy/0.12.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.12.0-orange)](https://crates.io/crates/bevy-svg/0.12.0) | [`bevy-0.12`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.12) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.11.0-orange)](https://crates.io/crates/bevy/0.11.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.11.0-orange)](https://crates.io/crates/bevy-svg/0.11.0) | [`bevy-0.11`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.11) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.10.0-orange)](https://crates.io/crates/bevy/0.10.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.10.1-orange)](https://crates.io/crates/bevy-svg/0.10.1) | [`bevy-0.10`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.10) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.9.0-orange)](https://crates.io/crates/bevy/0.9.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.9.0-orange)](https://crates.io/crates/bevy-svg/0.9.0) | [`bevy-0.9`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.9) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.8.0-orange)](https://crates.io/crates/bevy/0.8.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.8.0-orange)](https://crates.io/crates/bevy-svg/0.8.0) | [`bevy-0.8`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.8) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.7.0-orange)](https://crates.io/crates/bevy/0.7.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.7.0-orange)](https://crates.io/crates/bevy-svg/0.7.0) | [`bevy-0.7`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.7) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.6.0-orange)](https://crates.io/crates/bevy/0.6.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.6.0-orange)](https://crates.io/crates/bevy-svg/0.6.0) | [`bevy-0.6`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.6) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.5.0-orange)](https://crates.io/crates/bevy/0.5.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.4.0-orange)](https://crates.io/crates/bevy-svg/0.4.0) | [`bevy-0.5`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.5) |

</details>


## Examples

| Complex shapes       | Multiple colors | Fonts      |
|----------------------|-----------------|------------|
| ![complex_one_color] | ![two_colors]   | ![twinkle] |

[complex_one_color]: assets/readme/complex_one_color.png
[two_colors]: assets/readme/two_colors.png
[twinkle]: assets/readme/twinkle.png

## Usage

Copy this to your `Cargo.toml`

```toml
# Stable
bevy_svg = "0.16.0-rc1"

# 2D and 3D are available on default, if you only want/need one, use the following
bevy_svg = { version = "0.16.0-rc1", default-features = false, features = ["2d"] }
# or
bevy_svg = { version = "0.16.0-rc1", default-features = false, features = ["3d"] }

# Living on the edge (at your own risk 😅)
bevy_svg = { git = "https://github.com/Weasy666/bevy_svg", branch = "main" }
```

Then use it like this.

### 2D
```rust
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SVG Plugin".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("path/to/file.svg");
    commands.spawn((Camera2d::default(), Msaa::Sample4));
    commands.spawn((
        Svg2d(svg),
        Origin::Center, // Origin::TopLeft is the default
    ));
}
```

### 3D
```rust
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SVG Plugin".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("path/to/file.svg");
    commands.spawn((Camera3d::default(), Msaa::Sample4));
    commands.spawn((
        Svg3d(svg),
        Origin::Center, // Origin::TopLeft is the default
        Transform {
            translation: Vec3::new(0.0, 0.0, -600.0),
            // The scale you need depends a lot on your SVG and camera distance
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
    ));
}
```

## License

bevy_svg is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://choosealicense.com/licenses/mit)

[`Bevy`]: https://bevyengine.org
[`bevy_prototype_lyon`]: https://github.com/Nilirad/bevy_prototype_lyon
[`Lyon`]: https://github.com/nical/lyon
[`usvg`]: https://github.com/RazrFalcon/resvg
[`AssetLoader`]: https://docs.rs/bevy/0.14/bevy/asset/trait.AssetLoader.html
