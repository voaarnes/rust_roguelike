mod components {
    pub mod enemy;
    pub mod player;
    pub mod star;
}
mod resources {
    pub mod score;
    pub mod star_spawn_timer;
}
mod setup {
    pub mod camera;
    pub mod spawn;
}
mod systems {
    pub mod collisions;
    pub mod confine_enemy;
    pub mod confine_player;
    pub mod enemy_movement;
    pub mod player_movement;
    pub mod score;
    pub mod star_spawning;
    pub mod update_enemy_direction;
}
mod constants;

use bevy::prelude::*;
use constants::*;
use resources::score::Score;
use resources::star_spawn_timer::StarSpawnTimer;
use setup::*;
use systems::collisions;
use systems::score;
use systems::update_enemy_direction;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, camera::spawn_camera)
        .add_systems(Startup, spawn::spawn_player)
        .add_systems(Startup, spawn::spawn_enemies)
        .add_systems(Startup, spawn::spawn_stars)
        .add_systems(Update, player_movement::player_movement)
        .add_systems(Update, confine_player::confine_player_movement)
        .add_systems(Update, enemy_movement::enemy_movement)
        .add_systems(Update, update_enemy_direction::update_enemy_direction)
        .add_systems(Update, confine_enemy::confine_enemy_movement)
        .add_systems(Update, collisions::enemy_hit_player)
        .add_systems(Update, update_enemy_direction::player_hit_star)
        .add_systems(Update, score::update_score)
        .add_systems(Update, star_spawning::tick_star_spawn_timer)
        .add_systems(Update, star_spawning::spawn_stars_over_time)
        .run();
}
