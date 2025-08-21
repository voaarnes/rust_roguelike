mod core;
mod game;
mod ui;
mod world;
mod utils;

use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Roguelike - Survivor".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            core::CorePlugin,
            game::GamePlugin,
            ui::UIPlugin,
            world::WorldPlugin,
            utils::UtilsPlugin,
        ))
        .run();
}
