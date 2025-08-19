use bevy::prelude::*;
use crate::entities::player::Player;
use crate::tilemap::tilemap::MapSizePx;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}

pub fn camera_follow_player(
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut cam_q: Query<&mut Transform, With<MainCamera>>,
    map_size: Option<Res<MapSizePx>>,
    windows: Query<&Window>,
) {
    let Ok(player_tf) = player_q.get_single() else { return; };
    let Ok(mut cam_tf) = cam_q.get_single_mut() else { return; };
    let Some(map_size) = map_size else { return; };
    let Ok(window) = windows.get_single() else { return; };
    
    // Get viewport dimensions
    let half_width = window.width() * 0.5;
    let half_height = window.height() * 0.5;
    
    // Calculate map bounds for camera
    let half_map_w = map_size.w * 0.5;
    let half_map_h = map_size.h * 0.5;
    
    // Calculate camera bounds
    let min_x = -half_map_w + half_width;
    let max_x = half_map_w - half_width;
    let min_y = -half_map_h + half_height;
    let max_y = half_map_h - half_height;
    
    // Follow player but clamp to bounds
    if map_size.w > window.width() {
        cam_tf.translation.x = player_tf.translation.x.clamp(min_x, max_x);
    } else {
        cam_tf.translation.x = 0.0;
    }
    
    if map_size.h > window.height() {
        cam_tf.translation.y = player_tf.translation.y.clamp(min_y, max_y);
    } else {
        cam_tf.translation.y = 0.0;
    }
}
