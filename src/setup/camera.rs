use bevy::prelude::*;
use crate::entities::player::Player;
use crate::tilemap::tilemap::MapSizePx;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_camera(mut commands: Commands) {
    // This camera is only for the main menu
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}

pub fn spawn_game_camera(
    mut commands: Commands,
    query: Query<Entity, With<MainCamera>>,
) {
    // Despawn main menu camera first
    for entity in &query {
        commands.entity(entity).despawn();
    }
    
    // Spawn game camera
    commands.spawn((
        Camera2d::default(),
        GameCamera,
    ));
    
    info!("Game camera spawned");
}

pub fn cleanup_game_camera(
    mut commands: Commands,
    query: Query<Entity, With<GameCamera>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    
    // Respawn main menu camera
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}

pub fn camera_follow_player(
    player_q: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cam_q: Query<&mut Transform, With<GameCamera>>,
    map_size: Option<Res<MapSizePx>>,
    windows: Query<&Window>,
) {
    let Ok(player_tf) = player_q.single() else { 
        return; 
    };
    let Ok(mut cam_tf) = cam_q.single_mut() else { 
        return; 
    };
    let Some(map_size) = map_size else { 
        return; 
    };
    let Ok(window) = windows.single() else { 
        return; 
    };
    
    // Get viewport dimensions
    let half_width = window.width() * 0.5;
    let half_height = window.height() * 0.5;
    
    // Calculate map bounds for camera
    let half_map_w = map_size.w * 0.5;
    let half_map_h = map_size.h * 0.5;
    
    // Calculate camera bounds to prevent showing outside the map
    let min_x = -half_map_w + half_width;
    let max_x = half_map_w - half_width;
    let min_y = -half_map_h + half_height;
    let max_y = half_map_h - half_height;
    
    // Follow player but clamp to bounds
    if map_size.w > window.width() {
        cam_tf.translation.x = player_tf.translation.x.clamp(min_x, max_x);
    } else {
        // If map is smaller than window, center it
        cam_tf.translation.x = 0.0;
    }
    
    if map_size.h > window.height() {
        cam_tf.translation.y = player_tf.translation.y.clamp(min_y, max_y);
    } else {
        // If map is smaller than window, center it
        cam_tf.translation.y = 0.0;
    }
    
    // Keep z at camera distance
    cam_tf.translation.z = 999.9;
}

pub fn debug_positions(
    player_q: Query<&Transform, With<Player>>,
    cam_q: Query<&Transform, With<GameCamera>>,
    map_size: Option<Res<MapSizePx>>,
) {
    if let Ok(player_tf) = player_q.single() {
        if let Ok(cam_tf) = cam_q.single() {
            if let Some(map_size) = map_size {
                info!(
                    "Player: ({:.1}, {:.1}), Camera: ({:.1}, {:.1}), Map: {:.0}x{:.0}", 
                    player_tf.translation.x, 
                    player_tf.translation.y,
                    cam_tf.translation.x,
                    cam_tf.translation.y,
                    map_size.w,
                    map_size.h
                );
            }
        }
    }
}
