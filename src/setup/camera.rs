// camera.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    // Spawn a standard 2D camera; (0,0) is the center of the screen.
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
}

// ----- Camera follow -----
use crate::player::Player; // adjust the path to your Player component

pub fn camera_follow_player(
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut cam_q: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let Ok(mut cam_tf) = cam_q.get_single_mut() else { return; };

    cam_tf.translation.x = player_tf.translation.x;
    cam_tf.translation.y = player_tf.translation.y;
    // keep camera z as-is
}
