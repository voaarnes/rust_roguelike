use bevy::prelude::*;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

