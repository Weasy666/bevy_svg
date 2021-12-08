use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*
};
use bevy_svg::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "two_colors_visibility".to_string(),
            width: 400.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup.system())
        .add_system(keyboard_input_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SvgBuilder::from_file("examples/assets/neutron_star.svg")
            .origin(Origin::Center)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .is_visible(false)
            .build()
            .unwrap()
        );
}

/// This system toggles SVG visibility when 'V' is pressed
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (&Svg, &mut Visible),
    >,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        for (_, mut visible) in query.iter_mut() {
            visible.is_visible = !visible.is_visible;
        }
    }
}
