use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2d_bsn".to_string(),
                resolution: (600, 600).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((common::CommonPlugin, bevy_svg::prelude::SvgPlugin))
        .add_systems(Startup, scene.spawn())
        .run();
}

// `Svg2d` takes the asset path directly and `Origin` variants can be written inline.
fn scene() -> impl SceneList {
    let svg = bsn! {
        Svg2d("box.svg")
        Origin::Center
    };

    bsn_list! {
        Camera2d
        --
        @svg
    }
}
