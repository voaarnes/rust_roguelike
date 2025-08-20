pub mod camera;

use bevy::prelude::*;

pub use camera::spawn_camera;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::spawn_camera);
        app.add_systems(Update, camera::camera_follow_player);
    }
}
