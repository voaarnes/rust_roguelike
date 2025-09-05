use bevy::prelude::*;

#[cfg(target_os = "android")]
use bevy::window::WindowMode;

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .run();
}

// Remove the android_main function as it's not needed in Bevy 0.16
// Remove the run_app function with incorrect trait bounds

#[cfg(not(target_os = "android"))]
fn main() {
    // Your desktop main function
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
