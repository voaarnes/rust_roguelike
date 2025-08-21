use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct InputBuffer {
    pub buffer: VecDeque<InputAction>,
    pub max_size: usize,
    pub buffer_time: f32,
}

#[derive(Clone, Copy)]
pub struct InputAction {
    pub action: Action,
    pub timestamp: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Move(Vec2),
    Attack,
    Dash,
    UseAbility(u8),
    Interact,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            buffer: VecDeque::with_capacity(10),
            max_size: 10,
            buffer_time: 0.2,
        }
    }
}

pub fn buffer_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut buffer: ResMut<InputBuffer>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    // Clean old inputs
    buffer.buffer.retain(|action| {
        current_time - action.timestamp < buffer.buffer_time
    });
    
    // Add new inputs
    let mut movement = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { movement.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { movement.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { movement.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { movement.x += 1.0; }
    
    if movement != Vec2::ZERO {
        buffer.buffer.push_back(InputAction {
            action: Action::Move(movement.normalize()),
            timestamp: current_time,
        });
    }
    
    if keys.just_pressed(KeyCode::Space) {
        buffer.buffer.push_back(InputAction {
            action: Action::Attack,
            timestamp: current_time,
        });
    }
    
    if keys.just_pressed(KeyCode::ShiftLeft) {
        buffer.buffer.push_back(InputAction {
            action: Action::Dash,
            timestamp: current_time,
        });
    }
    
    // Trim buffer if too large
    while buffer.buffer.len() > buffer.max_size {
        buffer.buffer.pop_front();
    }
}
