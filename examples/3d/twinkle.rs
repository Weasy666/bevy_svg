use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3d_twinkle".to_string(),
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(Svg3dBundle {
        svg,
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(0.01, 0.01, 1.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
        ..Default::default()
    });
}
