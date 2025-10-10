use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "origin_check".to_string(),
                resolution: (600, 600).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((common::CommonPlugin, bevy_svg::prelude::SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("box.svg");
    commands.spawn(Camera2d);
    commands.spawn((Svg2d(svg.clone()), Origin::Center));
    commands.spawn((Svg2d(svg), Origin::TopLeft, common::DontChange));
}
