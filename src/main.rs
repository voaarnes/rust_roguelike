// src/main.rs
mod animation;
mod audio;
mod tilemap;
mod entities;
mod setup;
mod constants;
mod systems;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Roguelike".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            animation::AnimationPlugin,
            audio::AudioPlugin,
            tilemap::TilemapPlugin,
            entities::EntitiesPlugin,
            setup::SetupPlugin,
            systems::SystemsPlugin,
        ))
        .run();
}
