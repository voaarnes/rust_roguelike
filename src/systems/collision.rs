use bevy::prelude::*;
use crate::tilemap::tilemap::{Tile, DamageZone, Interactive, InteractionType};
use crate::entities::player::Player;

// Handle special tile interactions (damage zones, portals, etc.)
pub fn handle_tile_collisions(
    mut player_query: Query<(&Transform, &mut Player)>,
    tile_query: Query<(&Transform, &Tile, Option<&DamageZone>, Option<&Interactive>)>,
    time: Res<Time>,
    mut last_damage_time: Local<f32>,
) {
    *last_damage_time += time.delta_secs();
    
    for (player_transform, mut player) in player_query.iter_mut() {
        for (tile_transform, _tile, damage_zone, interactive) in tile_query.iter() {
            // Check if player is on this tile
            if is_player_on_tile(
                player_transform.translation.truncate(),
                tile_transform.translation.truncate(),
                Vec2::new(24.0, 24.0), // Player size
                Vec2::new(32.0, 32.0),  // Tile size
            ) {
                // Handle damage zones (spikes, lava)
                if let Some(damage) = damage_zone {
                    // Apply damage once per second
                    if *last_damage_time > 1.0 {
                        player.health -= damage.damage;
                        *last_damage_time = 0.0;
                        info!("Player took {} damage! Health: {}", damage.damage, player.health);
                    }
                }
                
                // Handle interactive tiles
                if let Some(interactive) = interactive {
                    match interactive.interaction_type {
                        InteractionType::Portal => {
                            info!("Player entered portal!");
                            // TODO: Implement level transition
                        },
                        InteractionType::Door => {
                            // Doors are walkable, so just for visual/audio feedback
                        },
                        InteractionType::Chest => {
                            // Chests should be opened with interaction key
                        },
                    }
                }
            }
        }
    }
}

fn is_player_on_tile(
    player_pos: Vec2,
    tile_pos: Vec2,
    player_size: Vec2,
    tile_size: Vec2,
) -> bool {
    let half_player = player_size / 2.0;
    let half_tile = tile_size / 2.0;
    
    let player_min = player_pos - half_player;
    let player_max = player_pos + half_player;
    let tile_min = tile_pos - half_tile;
    let tile_max = tile_pos + half_tile;
    
    // Check if player overlaps with tile
    !(player_max.x < tile_min.x || player_min.x > tile_max.x || 
      player_max.y < tile_min.y || player_min.y > tile_max.y)
}
