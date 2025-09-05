// Mobile-optimized camera system

use bevy::prelude::*;

#[derive(Component)]
pub struct MobileCamera;

pub fn setup_mobile_camera(mut commands: Commands) {
    // Spawn camera optimized for mobile portrait mode
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.5, // Zoom level for mobile
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        MobileCamera,
    ));
}

pub fn follow_player_smooth(
    player_query: Query<&Transform, (With<crate::Player>, Without<MobileCamera>)>,
    mut camera_query: Query<&mut Transform, With<MobileCamera>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let target = player_transform.translation.truncate();
            let current = camera_transform.translation.truncate();
            
            // Smooth camera follow with lerp
            let smoothing = 5.0; // Adjust for more/less smooth movement
            let new_pos = current.lerp(target, smoothing * time.delta_seconds());
            
            camera_transform.translation.x = new_pos.x;
            camera_transform.translation.y = new_pos.y;
        }
    }
}

// Adjust camera for screen orientation
pub fn handle_orientation_change(
    windows: Query<&Window>,
    mut projection_query: Query<&mut OrthographicProjection, With<MobileCamera>>,
) {
    if let Ok(window) = windows.get_single() {
        if let Ok(mut projection) = projection_query.get_single_mut() {
            let aspect_ratio = window.width() / window.height();
            
            // Adjust scale based on orientation
            if aspect_ratio < 1.0 {
                // Portrait mode
                projection.scale = 0.5;
            } else {
                // Landscape mode
                projection.scale = 0.3;
            }
        }
    }
}
