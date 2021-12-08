use bevy::prelude::*;
use bevy_svg::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "two_colors".to_string(),
            width: 400.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
    commands.spawn_bundle(SvgBuilder::from_file("examples/assets/neutron_star.svg")
            .origin(Origin::Center)
            .position(Vec3::new(0.0, 0.0, -1.0))
            .scale(Vec2::new(0.01, 0.01))
            .build()
            .unwrap()
        );
}
