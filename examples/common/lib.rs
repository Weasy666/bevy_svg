use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_svg::prelude::*;

/// Provides some common functionallity for all examples.
/// Like toggling visibility and through origin.
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_legend)
            .add_startup_system(setup_fps_counter)
            .add_startup_system(setup_origin_text)
            .add_system(keyboard_input_system)
            .add_system(fps_text_update_system)
            .add_system(origin_text_update_system);
    }
}

fn setup_legend(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Key Info",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nF",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " - Toggle Frame Diagnostics",
                TextStyle {
                    font: font_medium.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nO",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " - Cycle through Origins",
                TextStyle {
                    font: font_medium.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nV",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " - Toggle visibility",
                TextStyle {
                    font: font_medium.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    ));
}


#[derive(Component)]
pub struct DontChange;

/// This system toggles SVG visibility when 'V' is pressed and toggles through
/// origin when 'O' is pressed.
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut svg_query: Query<(&mut Origin, &mut Visibility), (With<Handle<Svg>>, Without<DontChange>)>,
    mut ui_query: Query<&mut Visibility, (With<Text>, Or<(With<FpsText>, With<OriginText>)>, Without<Handle<Svg>>)>,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        for (_, mut visible) in svg_query.iter_mut() {
            visible.is_visible = !visible.is_visible;
        }
    } else if keyboard_input.just_pressed(KeyCode::O) {
        for (mut origin, _) in svg_query.iter_mut() {
            *origin = match origin.as_ref() {
                Origin::BottomLeft => Origin::BottomRight,
                Origin::BottomRight => Origin::TopRight,
                Origin::Center => Origin::BottomLeft,
                Origin::TopLeft => Origin::Center,
                Origin::TopRight => Origin::TopLeft,
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::F) {
        for mut visible in &mut ui_query {
            visible.is_visible = !visible.is_visible;
        }
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Resource)]
struct FpsValues {
    min: f64,
    max: f64,
}

impl Default for FpsValues {
    fn default() -> Self {
        Self { min: 10000.0, max: 0.0 }
    }
}

fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 30.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                "\n(max: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 20.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                " - min: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 20.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                ")",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nms/frame: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 30.0,
                color: Color::GREEN,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}

fn fps_text_update_system(diagnostics: Res<Diagnostics>, mut fps_values: Local<FpsValues>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{fps_smoothed:.2}");
                fps_values.min = fps_values.min.min(fps_smoothed);
                text.sections[3].value = format!("{:.2}", fps_values.min);
                fps_values.max = fps_values.max.max(fps_smoothed);
                text.sections[5].value = format!("{:.2}", fps_values.max);
            }
        }
        if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(frame_time_smoothed) = frame_time.smoothed() {
                text.sections[8].value = format!("{frame_time_smoothed:.2}");
            }
        }
    }
}

#[derive(Component)]
struct OriginText;

fn setup_origin_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Origin: ",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 20.0,
                color: Color::GOLD,
            })
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        OriginText,
    ));
}

fn origin_text_update_system(
    mut text_query: Query<&mut Text, (With<OriginText>, Without<Origin>)>,
    query: Query<&Origin, Without<Text>>,
) {
    for mut text in &mut text_query {
        if let Some(origin) = query.iter().next() {
            text.sections[1].value = format!("{origin:?}");
        }
    }
}
