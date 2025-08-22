pub mod stage_manager;
pub mod stage_transition;

use bevy::prelude::*;

pub struct StagesPlugin;

impl Plugin for StagesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<stage_manager::StageManager>()
            .add_systems(Update, stage_transition::handle_stage_transitions);
    }
}
