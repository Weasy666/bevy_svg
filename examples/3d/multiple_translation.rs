use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3d_multiple_translation".to_string(),
                resolution: (600., 600.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((common::CommonPlugin, bevy_svg::prelude::SvgPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, svg_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("asteroid_field.svg");
    commands.spawn(Camera3d::default());
    commands.spawn((
        Svg3d(svg.clone()),
        Origin::Center,
        Transform {
            translation: Vec3::new(100.0, 0.0, -600.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..Default::default()
        },
        Direction::Up,
    ));

    let svg = asset_server.load("neutron_star.svg");
    commands.spawn((
        Svg3d(svg.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, -600.0),
            ..Default::default()
        },
        Direction::Up,
    ));
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn svg_movement(
    time: Res<Time>,
    mut svg_position: Query<(&mut Direction, &mut Transform), With<Svg3d>>,
) {
    for (mut direction, mut transform) in &mut svg_position {
        match *direction {
            Direction::Up => transform.translation.y += 150.0 * time.delta_secs(),
            Direction::Down => transform.translation.y -= 150.0 * time.delta_secs(),
        }

        if transform.translation.y > 200.0 {
            *direction = Direction::Down;
        } else if transform.translation.y < -200.0 {
            *direction = Direction::Up;
        }
    }
}
