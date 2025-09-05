// Touch input system for mobile controls

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TouchState {
    start_position: Option<Vec2>,
    current_position: Option<Vec2>,
    movement_threshold: f32,
}

#[derive(Event)]
pub struct MoveCommand {
    pub direction: Vec2,
}

impl TouchState {
    pub fn new() -> Self {
        Self {
            start_position: None,
            current_position: None,
            movement_threshold: 30.0, // Minimum drag distance in pixels
        }
    }
}

pub fn handle_touch_input(
    touches: Res<Touches>,
    mut touch_state: ResMut<TouchState>,
    mut move_events: EventWriter<MoveCommand>,
    _time: Res<Time>,
) {
    // Handle touch start
    if let Some(touch) = touches.iter_just_pressed().next() {
        touch_state.start_position = Some(touch.position());
        touch_state.current_position = Some(touch.position());
    }
    
    // Handle touch drag
    if let Some(touch) = touches.iter().next() {
        touch_state.current_position = Some(touch.position());
        
        if let (Some(start), Some(current)) = (touch_state.start_position, touch_state.current_position) {
            let delta = current - start;
            
            // Check if drag exceeds threshold
            if delta.length() > touch_state.movement_threshold {
                // Determine primary direction
                let direction = if delta.x.abs() > delta.y.abs() {
                    if delta.x > 0.0 {
                        Vec2::new(1.0, 0.0)  // Right
                    } else {
                        Vec2::new(-1.0, 0.0) // Left
                    }
                } else {
                    if delta.y > 0.0 {
                        Vec2::new(0.0, -1.0) // Down (inverted Y in screen space)
                    } else {
                        Vec2::new(0.0, 1.0)  // Up
                    }
                };
                
                // Send move command
                move_events.send(MoveCommand { direction });
                
                // Reset start position for continuous movement
                touch_state.start_position = Some(current);
            }
        }
    }
    
    // Handle touch end
    if touches.iter_just_released().count() > 0 {
        touch_state.start_position = None;
        touch_state.current_position = None;
    }
}

// Visual feedback for touch
pub fn render_touch_feedback(
    mut gizmos: Gizmos,
    touch_state: Res<TouchState>,
) {
    if let (Some(start), Some(current)) = (touch_state.start_position, touch_state.current_position) {
        // Draw line from start to current touch position
        gizmos.line_2d(start, current, Color::srgba(0.0, 1.0, 0.0, 0.5));
        gizmos.circle_2d(start, 10.0, Color::srgba(0.0, 1.0, 0.0, 0.3));
        gizmos.circle_2d(current, 8.0, Color::srgba(0.0, 1.0, 0.0, 0.6));
    }
}
