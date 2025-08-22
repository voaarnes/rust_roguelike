use bevy::prelude::*;
use crate::world::tilemap::{Tile, TileType};
use crate::game::movement::{Velocity, Collider};
use crate::game::player::Player;

pub struct TileCollisionPlugin;

impl Plugin for TileCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_tile_collisions);
    }
}

fn check_tile_collisions(
    mut player_query: Query<(&mut Transform, &Collider, &mut Velocity), With<Player>>,
    tile_query: Query<(&Transform, &Tile), (With<Tile>, Without<Player>)>,
) {
    let Ok((mut player_transform, player_collider, mut velocity)) = player_query.single_mut() else { return };
    
    let player_pos = player_transform.translation.truncate();
    let player_half_size = player_collider.size / 2.0;
    
    for (tile_transform, tile) in tile_query.iter() {
        // Skip walkable tiles
        if tile.walkable {
            continue;
        }
        
        let tile_pos = tile_transform.translation.truncate();
        let tile_half_size = Vec2::splat(16.0); // Half of 32x32 tile
        
        // Check collision
        if (player_pos.x - player_half_size.x < tile_pos.x + tile_half_size.x) &&
           (player_pos.x + player_half_size.x > tile_pos.x - tile_half_size.x) &&
           (player_pos.y - player_half_size.y < tile_pos.y + tile_half_size.y) &&
           (player_pos.y + player_half_size.y > tile_pos.y - tile_half_size.y) {
            
            // Calculate overlap
            let overlap_x = (player_half_size.x + tile_half_size.x) - (player_pos.x - tile_pos.x).abs();
            let overlap_y = (player_half_size.y + tile_half_size.y) - (player_pos.y - tile_pos.y).abs();
            
            // Resolve collision by moving player away from tile
            if overlap_x < overlap_y {
                // Horizontal collision
                if player_pos.x < tile_pos.x {
                    player_transform.translation.x = tile_pos.x - tile_half_size.x - player_half_size.x;
                } else {
                    player_transform.translation.x = tile_pos.x + tile_half_size.x + player_half_size.x;
                }
                velocity.0.x = 0.0;
            } else {
                // Vertical collision
                if player_pos.y < tile_pos.y {
                    player_transform.translation.y = tile_pos.y - tile_half_size.y - player_half_size.y;
                } else {
                    player_transform.translation.y = tile_pos.y + tile_half_size.y + player_half_size.y;
                }
                velocity.0.y = 0.0;
            }
        }
    }
}

pub fn is_position_walkable(
    position: Vec2,
    tile_query: &Query<(&Transform, &Tile), With<Tile>>,
) -> bool {
    for (tile_transform, tile) in tile_query.iter() {
        let tile_pos = tile_transform.translation.truncate();
        let tile_half_size = Vec2::splat(16.0);
        
        // Check if position is within this tile
        if (position.x >= tile_pos.x - tile_half_size.x) &&
           (position.x <= tile_pos.x + tile_half_size.x) &&
           (position.y >= tile_pos.y - tile_half_size.y) &&
           (position.y <= tile_pos.y + tile_half_size.y) {
            return tile.walkable;
        }
    }
    true // Default to walkable if no tile found
}
