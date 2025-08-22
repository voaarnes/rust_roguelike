mod core;
mod game;
mod ui;
mod world;

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
            core::CorePlugin,
            game::GamePlugin,
            world::WorldPlugin,
            ui::UIPlugin,
        ))
        .run();
}
