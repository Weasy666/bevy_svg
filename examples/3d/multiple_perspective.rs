use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3d_multiple_perspective".to_string(),
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
    let svg = asset_server.load("neutron_star.svg");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 175.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, -600.0), Vec3::Y),
        ..Default::default()
    });
    commands.spawn(Svg3dBundle {
        svg: svg.clone(),
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -600.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 3.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(Svg3dBundle {
        svg: svg.clone(),
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -700.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 3.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(Svg3dBundle {
        svg,
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -800.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 3.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
