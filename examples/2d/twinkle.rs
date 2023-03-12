use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2d_twinkle".to_string(),
                resolution: (600., 600.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(common::CommonPlugin)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("twinkle.svg");
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::Center,
        transform: Transform {
            scale: Vec3::new(0.75, 0.75, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
