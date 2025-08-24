pub mod state;
pub mod config;
pub mod events;
pub mod camera;
pub mod input;
pub mod save_system;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add game states
            .init_state::<state::GameState>()
            .init_state::<state::PlayState>()
            // Resources
            .init_resource::<config::GameConfig>()
            .init_resource::<input::InputBuffer>()
            .init_resource::<save_system::SaveData>()
            .init_resource::<state::GameStats>()
            // Events
            .add_event::<events::GameEvent>()
            .add_event::<events::PlayerEvent>()
            .add_event::<events::CombatEvent>()
            // Systems
            .add_systems(Startup, (
                camera::setup_camera,
                start_game,
            ))
            .add_systems(Update, (
                input::buffer_input_system,
                camera::camera_follow_player.run_if(in_state(state::GameState::Playing)),
                camera::camera_shake_system.run_if(in_state(state::GameState::Playing)).after(camera::camera_follow_player),
                save_system::auto_save_system,
                input::pause_game_system,
            ));
    }
}

fn start_game(mut next_state: ResMut<NextState<state::GameState>>) {
    // Auto-start the game
    next_state.set(state::GameState::Playing);
}
