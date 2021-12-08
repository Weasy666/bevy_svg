use bevy::prelude::*;
use bevy_svg::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "twinkle".to_string(),
            width: 400.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("twinkle.svg");
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
    let mut transform = Transform::from_xyz(0.0, 0.0, -1.0);
    transform.scale = Vec3::new(0.001, 0.001, 1.0);
    commands.spawn_bundle(SvgBundle {
        svg,
        origin: Origin::Center,
        transform,
        ..Default::default()
    });
}
