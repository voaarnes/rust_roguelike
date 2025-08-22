use bevy::prelude::*;

#[derive(Resource)]
pub struct StageManager {
    pub current_stage: usize,
    pub stages_completed: Vec<usize>,
}

impl Default for StageManager {
    fn default() -> Self {
        Self {
            current_stage: 1,
            stages_completed: Vec::new(),
        }
    }
}
