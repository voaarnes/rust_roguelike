
use bevy::prelude::{Vec2, UVec2};
// src/constants.rs
pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const NUMBER_OF_STARS: usize = 4;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;


pub const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0); // pixels per tile
pub const MAP_TILES: UVec2 = UVec2::new(40, 25);   // 40x25 tiles (tweak to taste)

pub fn map_size_px() -> Vec2 { TILE_SIZE * MAP_TILES.as_vec2() }
pub fn half_map_px() -> Vec2 { map_size_px() / 2.0 }
