use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}

use crate::entities::player::Player;

pub fn camera_follow_player(
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut cam_q: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player_tf) = player_q.single() else { return; };
    let Ok(mut cam_tf) = cam_q.single_mut() else { return; };

    cam_tf.translation.x = player_tf.translation.x;
    cam_tf.translation.y = player_tf.translation.y;
}
