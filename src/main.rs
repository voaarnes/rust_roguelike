mod core;
mod game;
mod ui;
mod world;
mod utils;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        // Configure window and renderer
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
        // Development diagnostics (remove in release)
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        // Core game plugins
        .add_plugins((
            core::CorePlugin,
            game::GamePlugin,
            ui::UIPlugin,
            world::WorldPlugin,
            utils::UtilsPlugin,
        ))
        .run();
}
