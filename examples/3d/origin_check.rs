use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "origin_check".to_string(),
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
    let svg = asset_server.load("box.svg");
    commands.spawn(Camera3dBundle::default());
    commands.spawn(Svg3dBundle {
        svg: svg.clone(),
        origin: Origin::Center,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -600.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((
        Svg3dBundle {
            svg,
            origin: Origin::TopLeft,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -600.0),
                ..Default::default()
            },
            ..Default::default()
        },
        common::DontChange
    ));
}
