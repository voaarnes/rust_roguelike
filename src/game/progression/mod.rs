use bevy::prelude::*;
use crate::core::state::{GameState, GameStats};
use crate::game::spawning::WaveManager;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameStats>()
            .add_systems(Update, (
                handle_experience,
                check_level_progression,
            ));
    }
}

fn handle_experience() {
    // Experience and leveling logic
}

fn check_level_progression(
    wave_manager: Res<WaveManager>,
    mut game_stats: ResMut<GameStats>,
    _next_state: ResMut<NextState<GameState>>,
) {
    // Progress to next level after defeating boss waves
    if wave_manager.current_wave > 0 && wave_manager.current_wave % 5 == 0 && wave_manager.wave_complete {
        game_stats.current_level += 1;
        println!("Progressed to level {}!", game_stats.current_level);
        
        // Could transition to a level selection screen or continue
        // For now, just continue playing
    }
}
