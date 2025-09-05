// Library entry point for Android

use bevy::prelude::*;

// Mark this as a placeholder for your existing Player component
#[derive(Component)]
pub struct Player;

pub mod systems;

#[cfg(target_os = "android")]
use bevy::window::WindowMode;

// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: bevy::winit::AndroidApp) {
    use bevy::winit::EventLoopBuilder;
    
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info),
    );
    
    let event_loop = EventLoopBuilder::new_android(android_app).build().unwrap();
    run_app(event_loop);
}

pub fn run_app(_event_loop: impl 'static) {
    let mut app = App::new();
    
    // Configure for mobile
    #[cfg(target_os = "android")]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Rust Roguelike".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    };
    
    #[cfg(not(target_os = "android"))]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Rust Roguelike".to_string(),
            ..default()
        }),
        ..default()
    };
    
    app.add_plugins(DefaultPlugins.set(window_plugin));
    
    // Add mobile systems on Android
    #[cfg(target_os = "android")]
    app.add_plugins(systems::mobile::MobilePlugin);
    
    // Add your existing game plugins here
    // app.add_plugins(YourGamePlugin);
    
    app.run();
}
