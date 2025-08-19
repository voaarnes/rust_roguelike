mod animation;
mod audio;
mod tilemap;
mod entities;
mod setup;
mod constants;

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
            animation::AnimationPlugin,
            audio::AudioPlugin,
            tilemap::TilemapPlugin,
            entities::EntitiesPlugin,
        ))
        .add_systems(Startup, setup::spawn_camera)
        .add_systems(Update, setup::camera::camera_follow_player)
        .run();
}
