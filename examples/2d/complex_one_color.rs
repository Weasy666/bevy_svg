use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "2d_complex_one_color".to_string(),
                width: 600.0,
                height: 600.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(common::CommonPlugin)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("asteroid_field.svg");
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::Center,
        transform: Transform { scale: Vec3::new(2.0, 2.0, 1.0), ..Default::default() },
        ..Default::default()
    });
}
