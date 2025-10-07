use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3d_complex_one_color".to_string(),
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
    let svg = asset_server.load("asteroid_field.svg");
    commands.spawn(Camera3d::default());
    commands.spawn((
        Svg3d(svg),
        Origin::Center,
        Transform {
            translation: Vec3::new(0.0, 0.0, -600.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..Default::default()
        },
    ));
}
