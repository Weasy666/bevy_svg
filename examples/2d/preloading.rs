use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "preloading".to_string(),
                resolution: (600., 600.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((common::CommonPlugin, bevy_svg::prelude::SvgPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, run)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Default, Eq, PartialEq)]
enum TutorialFsm {
    #[default]
    Ready,
    StartedLoad(Handle<Svg>),
    Wait(Handle<Svg>, u8),
    Loaded,
}

fn run(mut commands: Commands, asset_server: Res<AssetServer>, mut fsm: Local<TutorialFsm>) {
    match &*fsm {
        TutorialFsm::Ready => {
            let handle = asset_server.load("neutron_star.svg");
            *fsm = TutorialFsm::StartedLoad(handle);
        }
        TutorialFsm::StartedLoad(handle) => {
            if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
                *fsm = TutorialFsm::Wait(handle.clone(), 60);
            }
        }
        TutorialFsm::Wait(handle, frames) => {
            if *frames > 0 {
                *fsm = TutorialFsm::Wait(handle.clone(), *frames - 1);
            } else if let Some(svg) = asset_server.get_handle("neutron_star.svg") {
                commands.spawn(Svg2dBundle {
                    svg,
                    origin: Origin::Center,
                    ..Default::default()
                });

                *fsm = TutorialFsm::Loaded;
                dbg!("We loaded");
            }
        }
        TutorialFsm::Loaded => {}
    }
}
