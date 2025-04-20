use bevy::prelude::*;

use crate::components::player::Player;
use crate::PLAYER_SPEED;

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if let Ok(mut transform) = player_query.get_single_mut() {
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}
