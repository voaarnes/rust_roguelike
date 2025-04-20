use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::components::star::Star;
use crate::resources::star_spawn_timer::StarSpawnTimer;
use crate::{STAR_SIZE, STAR_SPAWN_TIME};

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x: f32 = random::<f32>() * window.height();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/player_x.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Star {},
        ));
    }
}
