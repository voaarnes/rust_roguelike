use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Camera spawn
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}
