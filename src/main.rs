/*!
 * Rust Roguelike - Survivor
 * 
 * A Vampire Survivors-inspired roguelike game built with Bevy 0.16.
 * Players collect fruit power-ups to survive waves of enemies.
 */

mod core;
mod game;
mod entities; // Bridge module for compatibility
mod ui;
mod world;
mod utils;
mod stages;
mod setup;
mod states;
mod systems;

use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Roguelike - Survivor".into(),
                resolution: (360.0, 640.0).into(), // Phone dimensions (9:16 aspect ratio)
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
            stages::StagesPlugin,
            states::StatesPlugin,
            systems::GameSystemsPlugin,
        ))
        .run();
}

