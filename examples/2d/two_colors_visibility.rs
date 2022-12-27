use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use bevy_svg::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "two_colors_visibility".to_string(),
                width: 400.0,
                height: 400.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("neutron_star.svg");
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::Center,
        visibility: Visibility { is_visible: false },
        ..Default::default()
    });
}

/// This system toggles SVG visibility when 'V' is pressed
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Handle<Svg>, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        for (_, mut visible) in query.iter_mut() {
            visible.is_visible = !visible.is_visible;
        }
    }
}
