use bevy::prelude::*;

pub mod touch_input;

pub struct MobilePlugin;

impl Plugin for MobilePlugin {
    fn build(&self, app: &mut App) {
        // Combine all mobile plugin functionality here
        app.add_systems(Update, (
            touch_input::handle_touch_input,
            // Add other mobile systems
        ));
        
        // Add any mobile-specific resources or events
    }
}
