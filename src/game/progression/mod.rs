use bevy::prelude::*;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_experience);
    }
}

fn handle_experience() {
    // Experience and leveling logic
}
