use bevy::prelude::*;
use std::collections::HashMap;

pub struct AudioPlugin;
use bevy::audio::Volume; 

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioManager>()
            .add_systems(Startup, setup_audio);
    }
}

#[derive(Resource)]
pub struct AudioManager {
    pub sounds: HashMap<String, Handle<AudioSource>>,
    pub music: HashMap<String, Handle<AudioSource>>,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            sounds: HashMap::new(),
            music: HashMap::new(),
        }
    }
}

impl AudioManager {
    pub fn play_sound(
        &self,
        commands: &mut Commands,
        sound_name: &str,
        volume: f32,
    ) {
        if let Some(sound) = self.sounds.get(sound_name) {
            commands.spawn((
                AudioPlayer::new(sound.clone()),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(volume))
                
            ));
        }
    }

    pub fn play_music(
        &self,
        commands: &mut Commands,
        music_name: &str,
        volume: f32,
    ) {
        if let Some(music) = self.music.get(music_name) {
            commands.spawn((
                AudioPlayer::new(music.clone()),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(volume))
            ));
        }
    }
}

fn setup_audio(
    mut audio_manager: ResMut<AudioManager>,
    asset_server: Res<AssetServer>,
) {
    // Load sound effects
    audio_manager.sounds.insert(
        "hit".to_string(),
        asset_server.load("audio/audio_001.ogg"),
    );
    audio_manager.sounds.insert(
        "collect".to_string(),
        asset_server.load("audio/audio_002.ogg"),
    );
    
    // Load music tracks
    // audio_manager.music.insert(
    //     "main_theme".to_string(),
    //     asset_server.load("audio/main_theme.ogg"),
    // );
}
