// Mobile systems module

pub mod touch_input;
pub mod camera;

use bevy::prelude::*;
use touch_input::*;
use camera::*;

pub struct MobilePlugin;

impl Plugin for MobilePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TouchState>()
            .add_event::<MoveCommand>()
            .add_systems(Startup, setup_mobile_camera)
            .add_systems(Update, (
                handle_touch_input,
                render_touch_feedback,
                follow_player_smooth,
                handle_orientation_change,
            ));
    }
}

pub mod movement_adapter;

// Update the plugin implementation to include movement adapter
impl Plugin for MobilePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TouchState>()
            .add_event::<MoveCommand>()
            .add_systems(Startup, setup_mobile_camera)
            .add_systems(Update, (
                handle_touch_input,
                render_touch_feedback,
                follow_player_smooth,
                handle_orientation_change,
                movement_adapter::process_move_commands,
            ));
    }
}
