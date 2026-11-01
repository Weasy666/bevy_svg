use bevy::prelude::*;
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, draw_center_marker))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3d::default());
    commands.spawn((
        Svg3d(asset_server.load("box.svg")),
        Origin::Center,
        Transform::from_xyz(0.0, 0.0, -300.0),
    ));
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Svg3d>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs());
    }
}

fn draw_center_marker(
    mut gizmos: Gizmos,
    query_global: Query<&GlobalTransform, With<Svg3d>>,
    query_local: Query<&Transform, With<Svg3d>>,
) {
    for transform in &query_global {
        gizmos.cross(
            Isometry3d::from_translation(transform.translation()),
            10.0,
            Color::hsv(40., 1.0, 1.0),
        );
    }

    for transform in &query_local {
        gizmos.cross(
            Isometry3d::from_translation(transform.translation),
            10.0,
            Color::WHITE,
        );
    }
}
