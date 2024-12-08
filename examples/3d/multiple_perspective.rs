use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3d_multiple_perspective".to_string(),
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
    let svg = asset_server.load("neutron_star.svg");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(100.0, 175.0, 0.0).looking_at(Vec3::new(0.0, 0.0, -600.0), Vec3::Y),
    ));
    commands.spawn((
        Svg3d(svg.clone()),
        Origin::Center,
        Transform {
            translation: Vec3::new(0.0, 0.0, -600.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 3.0),
            ..Default::default()
        },
    ));
    commands.spawn((
        Svg3d(svg.clone()),
        Origin::Center,
        Transform {
            translation: Vec3::new(0.0, 0.0, -700.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 3.0),
            ..Default::default()
        },
    ));
    commands.spawn((
        Svg3d(svg),
        Origin::Center,
        Transform {
            translation: Vec3::new(0.0, 0.0, -800.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 3.0),
            ..Default::default()
        },
    ));
}
