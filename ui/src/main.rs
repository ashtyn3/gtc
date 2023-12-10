use bevy::{prelude::*, render::view::RenderLayers};
use bevy_mod_picking::DefaultPickingPlugins;
use components::*;
use events::{game_exit, state, take, TakeEvent};
use menu::MenuPlugin;
use proc::CmdRunEvent;
use resources::Board;

mod components;
mod events;
mod init;
mod menu;
mod proc;
mod resources;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(MenuPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Game), proc::Instance::gtc_startup)
        .add_systems(
            OnEnter(GameState::Game),
            (init::create_board, init::create_pieces),
        )
        .add_systems(OnEnter(GameState::Game), init::create_ui)
        .add_systems(
            OnExit(GameState::Game),
            (despawn_screen::<GameEntity>, game_exit),
        )
        .add_systems(Update, (take, state).run_if(in_state(GameState::Game)))
        .insert_resource(Board(vec![]))
        .add_plugins(DefaultPickingPlugins)
        .add_event::<TakeEvent>()
        .add_event::<CmdRunEvent>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Goats and Tigers (the game)".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run()
}

fn setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.camera.hdr = true;
    commands.spawn((cam, RenderLayers::from_layers(&[0, 1, 2, 3])));
}
