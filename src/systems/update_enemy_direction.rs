use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::enemy::Enemy;
use crate::components::player::Player;
use crate::components::star::Star;
use crate::resources::score::Score;
use crate::ENEMY_SIZE;
use crate::PLAYER_SIZE;
use crate::STAR_SIZE;

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            // Audio code commented out
            // let sound_effect_1: Handle<AudioSource> = asset_server.load("audio/audio_001.ogg");
            // let sound_effect_2: Handle<AudioSource> = asset_server.load("audio/audio_002.ogg");

            // let sound_effect: Handle<AudioSource> = if random::<f32>() > 0.5 {
            //     sound_effect_1
            // } else {
            //     sound_effect_2
            // };

            //audio.play(sound_effect);
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star!");
                score.value += 1;
                // let sound_effect: Handle<AudioSource> = asset_server.load("audio/audio_001.ogg");
                // audio.play(sound_effect);
                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/audio_001.ogg")),
                    PlaybackSettings::ONCE,
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}
