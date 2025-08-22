use bevy::prelude::*;

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_abilities);
    }
}

fn handle_abilities() {
    // Abilities logic
}
