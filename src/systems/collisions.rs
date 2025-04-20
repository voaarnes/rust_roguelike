use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::prelude::*;

use crate::components::{enemy::Enemy, player::Player, star::Star};
use crate::{ENEMY_SIZE, PLAYER_SIZE, STAR_SIZE};

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance: f32 = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let enemy_radius: f32 = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                // let sound_effect: Handle<AudioSource> = asset_server.load("audio/audio_001");
                // audio.play(sound_effect);
                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/audio_001.ogg")),
                    PlaybackSettings::ONCE,
                ));
                commands.entity(player_entity).despawn();
            }
        }
    }
}
