use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "origin_check".to_string(),
                resolution: (600., 600.).into(),
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
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Svg2dBundle {
        svg: svg.clone(),
        origin: Origin::Center,
        ..Default::default()
    });
    commands.spawn((
        Svg2dBundle {
            svg,
            origin: Origin::TopLeft,
            ..Default::default()
        },
        common::DontChange,
    ));
}
