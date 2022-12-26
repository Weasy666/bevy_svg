# bevy_svg
[![Crates.io](https://img.shields.io/crates/v/bevy_svg.svg)](https://crates.io/crates/bevy_svg)
[![license](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE)

For one of my personal projects i needed a way to load and display some simple SVG files/shapes in [`Bevy`],
so i took inspiration from [`bevy_prototype_lyon`] and modified and extended it to...well...load and display
simple SVG files. SVGs can be used/displayed in `2D` as well as in `3D`.

Files are loaded through [`AssetLoader`], then parsed and simplified with [`usvg`] and then tessellated with [`Lyon`]
into a vertex buffer, which lastly is convert into a [`Bevy`] mesh and drawn with custom [shaders].

> *Note:* The SVG support is currently rather basic, i'd like to expand that in the future.


## Compatibility
| `Bevy` version | `bevy_svg` version | Branch      |
|----------------|--------------------|-------------|
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.8.0-orange)](https://crates.io/crates/bevy/0.8.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.8.0-orange)](https://crates.io/crates/bevy-svg/0.8.0) | [`bevy-0.8`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.8) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.7.0-orange)](https://crates.io/crates/bevy/0.7.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.7.0-orange)](https://crates.io/crates/bevy-svg/0.7.0) | [`bevy-0.7`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.7) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.6.0-orange)](https://crates.io/crates/bevy/0.6.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.6.0-orange)](https://crates.io/crates/bevy-svg/0.6.0) | [`bevy-0.6`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.6) |
| [![Crates.io](https://img.shields.io/badge/crates.io-v0.5.0-orange)](https://crates.io/crates/bevy/0.5.0) | [![Crates.io](https://img.shields.io/badge/crates.io-v0.4.0-orange)](https://crates.io/crates/bevy-svg/0.4.0) | [`bevy-0.5`](https://github.com/Weasy666/bevy_svg/tree/bevy-0.5) |
| [![Crates.io](https://img.shields.io/badge/branch-main-yellow)](https://github.com/bevyengine/bevy) | [![Crates.io](https://img.shields.io/badge/branch-main-yellow)](https://github.com/Weasy666/bevy_svg/) | [`main`](https://github.com/Weasy666/bevy_svg) |


## Examples

| Complex shapes       | Multiple colors | Fonts      |
|----------------------|-----------------|------------|
| ![complex_one_color] | ![two_colors]   | ![twinkle] |

[complex_one_color]: assets/complex_one_color.png
[two_colors]: assets/two_colors.png
[twinkle]: assets/twinkle.png

## Usage

Copy this to your `Cargo.toml`

```toml
# Stable
bevy_svg = "0.8"

# 2D and 3D are available on default, if you only want/need one, use the following
bevy_svg = { version = "0.8", default-features = false, features = "2d" }
# or
bevy_svg = { version = "0.8", default-features = false, features = "3d" }

# Living on the edge (at your own risk 😅)
bevy_svg = { git = "https://github.com/Weasy666/bevy_svg", branch = "main" }
```

Then use it like this.

### 2D
```rust
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
            title: "SVG Plugin".to_string(),
            ..Default::default()
        },
            ..Default::default()
        }))
                .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("path/to/file.svg");
    commands.spawn(OrthographicCameraBundle::new_2d());
    commands.spawn(SvgBundle {
        svg,
        origin: Origin::Center, // Origin::TopLeft is the default
        ..Default::default()
    });
}
```

### 3D
```rust
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
            title: "SVG Plugin".to_string(),
            ..Default::default()
        },
            ..Default::default()
        }))
                .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("path/to/file.svg");
    commands.spawn(PerspectiveCameraBundle::new_3d());
    commands.spawn(SvgBundle {
        svg,
        origin: Origin::Center, // Origin::TopLeft is the default
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            scale: Vec3::new(0.05, 0.05, 1.0), // The scale you need depends a lot on your SVG and camera distance
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
        ..Default::default()
    });
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
[`AssetLoader`]: https://docs.rs/bevy/0.7/bevy/asset/trait.AssetLoader.html
