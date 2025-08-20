pub mod stage_manager;
pub mod stage_transition;

use bevy::prelude::*;

pub struct StagesPlugin;

impl Plugin for StagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            stage_manager::StageManagerPlugin,
            stage_transition::StageTransitionPlugin,
        ));
    }
}
