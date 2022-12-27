use bevy::prelude::*;
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "twinkle".to_string(),
                width: 400.0,
                height: 400.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("twinkle.svg");
    commands.spawn(Camera2dBundle::default());
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.scale = Vec3::new(0.75, 0.75, 1.0);
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::Center,
        transform,
        ..Default::default()
    });
}
