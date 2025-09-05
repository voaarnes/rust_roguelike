// Adapter to connect touch input to existing movement system

use bevy::prelude::*;
use super::touch_input::MoveCommand;

// Adapt touch commands to existing movement system
pub fn process_move_commands(
    mut move_events: EventReader<MoveCommand>,
    mut player_query: Query<&mut Transform, With<crate::Player>>,
    time: Res<Time>,
) {
    for event in move_events.read() {
        if let Ok(mut transform) = player_query.get_single_mut() {
            // Move player based on touch direction
            let movement_speed = 200.0; // Adjust speed as needed
            let movement = event.direction * movement_speed * time.delta_seconds();
            
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}
