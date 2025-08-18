// src/main.rs - Simplified version that will compile
mod animation;
mod audio;
mod tilemap;
mod entities;
mod setup;
mod constants;

use bevy::prelude::*;
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Roguelike".to_string(),
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
        ))
        .add_systems(Startup, setup::camera::spawn_camera)
        .run();
}

// For now, remove src/lib.rs or rename it to src/lib.rs.bak
