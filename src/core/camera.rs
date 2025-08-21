use bevy::prelude::*;
use bevy::math::Rect;

#[derive(Component)]
pub struct MainCamera {
    pub smoothing: f32,
    pub offset: Vec2,
    pub bounds: Option<Rect>,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            smoothing: 5.0,
            offset: Vec2::ZERO,
            bounds: None,
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
     mut cam_q: Query<(&mut Transform, &MainCamera), (With<Camera>, Without<crate::game::player::Player>)>,
     time: Res<Time>,
 ) {
     let Ok(player_tf) = player_q.single() else { return };
     let Ok((mut cam_tf, cam)) = cam_q.single_mut() else { return };
    
    let target = player_tf.translation.truncate() + cam.offset;
    let current = cam_tf.translation.truncate();
    
    let new_pos = current.lerp(target, cam.smoothing * time.delta_secs());
    
    let final_pos = if let Some(bounds) = cam.bounds {
        Vec2::new(
            new_pos.x.clamp(bounds.min.x, bounds.max.x),
            new_pos.y.clamp(bounds.min.y, bounds.max.y),
        )
    } else {
        new_pos
    };
    
    cam_tf.translation.x = final_pos.x;
    cam_tf.translation.y = final_pos.y;
}

pub fn camera_shake_system(
    mut cam_q: Query<(Entity, &mut Transform, &mut CameraShake)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut shake) in cam_q.iter_mut() {
        shake.duration.tick(time.delta());
        
        if !shake.duration.finished() {
            let progress = shake.duration.fraction_remaining();
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * shake.intensity * progress,
                (rand::random::<f32>() - 0.5) * shake.intensity * progress,
            );
            transform.translation += offset.extend(0.0);
        } else {
            commands.entity(entity).remove::<CameraShake>();
        }
    }
}
