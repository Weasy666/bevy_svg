# bevy_svg
[![Crates.io](https://img.shields.io/crates/v/bevy_svg.svg)](https://crates.io/crates/bevy_svg)
[![license](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE)

For one of my personal projects i needed a way to load and display some simple SVG files/shapes in [`Bevy`],
so i took inspiration from [`bevy_prototype_lyon`] and modified and extended it to...well...load and display
simple SVG files. SVGs can be used/displayed in `2D` as well as in `3D`.

Files are loaded through [`AssetLoader`], then parsed and simplified with [`usvg`] and then tessellated with [`Lyon`]
into a vertex buffer, which lastly is convert into a [`Bevy`] mesh and drawn with custom [shaders].

[shaders]: src/plugin.rs#L91-L119


## Compatibility
| `Bevy` version | `bevy_svg` version | Branch      |
|--------------|---------------|-------------|
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

This crate is not yet on crates.io because it uses Bevy master. But i am planning to publish it as soon as Bevy 0.5 is released.
Until then, you need to copy this to your `Cargo.toml`

```toml
# Stable
bevy_svg = "0.3"

# Living on the edge (at your own risk 😅)
bevy_svg = { git = "https://github.com/Weasy666/bevy_svg", branch = "main" }
```

Then use it like this.

### 2D
```rust
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "SVG Plugin".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup.system());
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("path/to/file.svg");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SvgBundle {
        svg,
        origin: Origin::Center,
        ..Default::default()
    });
}
```

### 3D
```rust
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "SVG Plugin".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup.system());
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("path/to/file.svg");
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
    commands.spawn_bundle(SvgBundle {
        svg,
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            scale: Vec3::new(0.05, 0.05, 1.0), // The scale depends a lot on your SVG and camera distance
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
        ..Default::default()
    });
}
```


[`Bevy`]: https://bevyengine.org
[`bevy_prototype_lyon`]: https://github.com/Nilirad/bevy_prototype_lyon
[`Lyon`]: https://github.com/nical/lyon
[`usvg`]: https://github.com/RazrFalcon/resvg
[`AssetLoader`]: https://docs.rs/bevy/0.5.0/bevy/asset/trait.AssetLoader.html
