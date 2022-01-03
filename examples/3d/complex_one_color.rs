use bevy::prelude::*;
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "complex_one_color".to_string(),
            width: 400.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("asteroid_field.svg");
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
    commands.spawn_bundle(Svg3dBundle {
        svg,
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(0.005, 0.005, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}