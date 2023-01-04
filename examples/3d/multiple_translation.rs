use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "3d_multiple_translation".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        },
        ..Default::default()
    }))
        .add_plugin(common::CommonPlugin)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .add_system(svg_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("asteroid_field.svg");
    commands.spawn(Camera3dBundle::default());
    commands.spawn((
        Svg3dBundle {
            svg,
            origin: Origin::Center,
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, -600.0),
                scale: Vec3::new(2.0, 2.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Direction::Up
    ));

    let svg = asset_server.load("neutron_star.svg");
    commands.spawn((
        Svg3dBundle {
            svg,
            origin: Origin::Center,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -600.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Direction::Up
    ));
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn svg_movement(time: Res<Time>, mut svg_position: Query<(&mut Direction, &mut Transform), With<Handle<Svg>>>) {
    for (mut direction, mut transform) in &mut svg_position {
        match *direction {
            Direction::Up => transform.translation.y += 150.0 * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150.0 * time.delta_seconds(),
        }

        if transform.translation.y > 200.0 {
            *direction = Direction::Down;
        } else if transform.translation.y < -200.0 {
            *direction = Direction::Up;
        }
    }
}
