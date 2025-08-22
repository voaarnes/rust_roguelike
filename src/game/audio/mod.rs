use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>();
    }
}

#[derive(Resource, Default)]
pub struct AudioAssets {
    pub background_music: Vec<Handle<AudioSource>>,
    pub sound_effects: Vec<Handle<AudioSource>>,
}
