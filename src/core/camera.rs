use bevy::prelude::*;
use bevy::math::Rect;

#[derive(Component)]
pub struct MainCamera {
    pub smoothing: f32,
    pub offset: Vec2,
    pub bounds: Option<Rect>,
    pub base_position: Vec2, // Store the intended position before shake
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            smoothing: 8.0, // More responsive camera for mobile
            offset: Vec2::ZERO,
            bounds: None,
            base_position: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: Timer,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera::default(),
    ));
}

pub fn camera_follow_player(
     player_q: Query<&Transform, (With<crate::game::player::Player>, Without<Camera>)>,
     mut cam_q: Query<(&mut Transform, &mut MainCamera, Option<&CameraShake>), (With<Camera>, Without<crate::game::player::Player>)>,
     time: Res<Time>,
 ) {
     let Ok(player_tf) = player_q.single() else { return };
     let Ok((mut cam_tf, mut cam, has_shake)) = cam_q.single_mut() else { return };
    
    let target = player_tf.translation.truncate() + cam.offset;
    let current = cam.base_position;
    
    let new_pos = current.lerp(target, cam.smoothing * time.delta_secs());
    
    let final_pos = if let Some(bounds) = cam.bounds {
        Vec2::new(
            new_pos.x.clamp(bounds.min.x, bounds.max.x),
            new_pos.y.clamp(bounds.min.y, bounds.max.y),
        )
    } else {
        new_pos
    };
    
    // Store the base position (without shake)
    cam.base_position = final_pos;
    
    // Only set camera position if there's no active shake
    // (shake system will handle position when shaking)
    if has_shake.is_none() {
        cam_tf.translation.x = final_pos.x;
        cam_tf.translation.y = final_pos.y;
    }
}

pub fn camera_shake_system(
    mut cam_q: Query<(Entity, &mut Transform, &mut CameraShake, &MainCamera)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut shake, main_cam) in cam_q.iter_mut() {
        shake.duration.tick(time.delta());
        
        if !shake.duration.finished() {
            let progress = shake.duration.fraction_remaining();
            let shake_offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * shake.intensity * progress * 10.0,
                (rand::random::<f32>() - 0.5) * shake.intensity * progress * 10.0,
            );
            
            // Apply shake on top of base position
            transform.translation.x = main_cam.base_position.x + shake_offset.x;
            transform.translation.y = main_cam.base_position.y + shake_offset.y;
        } else {
            // Reset to base position when shake is done
            transform.translation.x = main_cam.base_position.x;
            transform.translation.y = main_cam.base_position.y;
            commands.entity(entity).remove::<CameraShake>();
        }
    }
}
