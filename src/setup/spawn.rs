use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::components::{enemy::Enemy, player::Player, star::Star};
use crate::NUMBER_OF_ENEMIES;
use crate::NUMBER_OF_STARS;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/player_x.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/player_x.png"),
                ..default()
            },
            Transform::from_xyz(rand_x, rand_y, 0.0),
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

// Initial star spawning, not routine bonus spawns.
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/player_x.png"),
                ..default()
            },
            Transform::from_xyz(rand_x, rand_y, 0.0),
            Star {},
        ));
    }
}
