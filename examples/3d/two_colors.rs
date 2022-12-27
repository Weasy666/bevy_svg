use bevy::prelude::*;
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "two_colors".to_string(),
            width: 400.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("neutron_star.svg");
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(Svg3dBundle {
        svg: svg.clone(),
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            scale: Vec3::new(0.05, 0.05, 1.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
        ..Default::default()
    });
    commands.spawn_bundle(Svg3dBundle {
        svg: svg.clone(),
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(0.05, 0.05, 1.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
        ..Default::default()
    });
    commands.spawn_bundle(Svg3dBundle {
        svg,
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.5),
            scale: Vec3::new(0.05, 0.05, 1.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
        },
        ..Default::default()
    });
}
