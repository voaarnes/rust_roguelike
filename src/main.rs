mod animation;
mod audio;
mod tilemap;
mod entities;
mod setup;
mod constants;
mod states;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Roguelike".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            states::StatesPlugin,
            animation::AnimationPlugin,
            audio::AudioPlugin,
            tilemap::TilemapPlugin,
            entities::EntitiesPlugin,
            ui::UIPlugin,
        ))
        .add_systems(Startup, setup::spawn_camera)
        .add_systems(OnEnter(states::GameState::InGame), setup::camera::spawn_game_camera)
        .add_systems(OnExit(states::GameState::InGame), setup::camera::cleanup_game_camera)
        .add_systems(
            Update, 
            (
                setup::camera::camera_follow_player,
                // Uncomment the next line to see debug info about positions
                // setup::camera::debug_positions,
            ).run_if(in_state(states::GameState::InGame))
        )
        .run();
}
