use bevy::prelude::*;
use crate::game::player::{Player, PlayerController};
use crate::game::enemy::Enemy;
use crate::game::combat::Health;
use crate::game::movement::Velocity;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TilemapConfig>()
            .add_systems(Startup, crate::world::level_loader::load_level)
            .add_systems(Update, (
                apply_tile_effects,
                apply_water_effects,
            ));
    }
}

#[derive(Resource)]
pub struct TilemapConfig {
    pub tile_size: f32,
    pub tileset_columns: usize,
    pub tileset_rows: usize,
}

impl Default for TilemapConfig {
    fn default() -> Self {
        Self {
            tile_size: 32.0,
            tileset_columns: 16,
            tileset_rows: 16,
        }
    }
}

#[derive(Component)]
pub struct AnimatedTile {
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: Timer,
}

#[derive(Component, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
    pub tile_index: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Chest,
    Spike,
    Water,
    Portal,
    Lava,
    Grass,
    Stone,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        let walkable = match tile_type {
            TileType::Floor | TileType::Door | TileType::Grass | TileType::Stone => true,
            TileType::Wall | TileType::Chest | TileType::Lava => false,
            TileType::Spike | TileType::Portal | TileType::Water => true, // Water is now walkable
        };
        
        let tile_index = match tile_type {
            TileType::Floor => 1,
            TileType::Grass => 2,
            TileType::Stone => 3,
            TileType::Wall => 17,
            TileType::Door => 33,
            TileType::Chest => 37,
            TileType::Spike => 41,
            TileType::Water => 45,
            TileType::Lava => 49,
            TileType::Portal => 53,
        };
        
        Self {
            tile_type,
            walkable,
            tile_index,
        }
    }
}

#[derive(Component)]
pub struct InWater {
    pub depth: f32, // 0.0 to 1.0, where 1.0 is fully submerged
}

#[derive(Component)]
pub struct OnSpikes;

// System to handle spike damage and visual effects
fn apply_tile_effects(
    mut commands: Commands,
    tile_query: Query<(&Transform, &Tile), With<Tile>>,
    mut player_query: Query<(Entity, &Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
    spike_query: Query<Entity, With<OnSpikes>>,
    time: Res<Time>,
) {
    let tile_size = 32.0;
    let half_tile = tile_size / 2.0;
    
    // Check players on spikes
    for (player_entity, player_transform, mut player_health) in player_query.iter_mut() {
        // Skip dead players
        if player_health.is_dead() {
            continue;
        }
        
        let player_pos = player_transform.translation.truncate();
        let mut on_spikes = false;
        
        for (tile_transform, tile) in tile_query.iter() {
            if tile.tile_type == TileType::Spike {
                let tile_pos = tile_transform.translation.truncate();
                let distance = player_pos.distance(tile_pos);
                
                if distance < half_tile {
                    on_spikes = true;
                    // Deal damage every second when on spikes
                    player_health.current -= (15.0 * time.delta_secs()) as i32;
                    break;
                }
            }
        }
        
        // Add or remove OnSpikes component safely
        if on_spikes && !spike_query.contains(player_entity) {
            if let Ok(mut entity_commands) = commands.get_entity(player_entity) {
                entity_commands.insert(OnSpikes);
            }
        } else if !on_spikes && spike_query.contains(player_entity) {
            if let Ok(mut entity_commands) = commands.get_entity(player_entity) {
                entity_commands.remove::<OnSpikes>();
            }
        }
    }
    
    // Check enemies on spikes
    for (enemy_entity, enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
        // Skip dead enemies
        if enemy_health.is_dead() {
            continue;
        }
        
        let enemy_pos = enemy_transform.translation.truncate();
        let mut on_spikes = false;
        
        for (tile_transform, tile) in tile_query.iter() {
            if tile.tile_type == TileType::Spike {
                let tile_pos = tile_transform.translation.truncate();
                let distance = enemy_pos.distance(tile_pos);
                
                if distance < half_tile {
                    on_spikes = true;
                    // Deal damage to enemies on spikes too
                    enemy_health.current -= (10.0 * time.delta_secs()) as i32;
                    break;
                }
            }
        }
        
        if on_spikes && !spike_query.contains(enemy_entity) {
            if let Ok(mut entity_commands) = commands.get_entity(enemy_entity) {
                entity_commands.insert(OnSpikes);
            }
        } else if !on_spikes && spike_query.contains(enemy_entity) {
            if let Ok(mut entity_commands) = commands.get_entity(enemy_entity) {
                entity_commands.remove::<OnSpikes>();
            }
        }
    }
}

// System to handle water effects (slower movement, visual depth)
fn apply_water_effects(
    mut commands: Commands,
    tile_query: Query<(&Transform, &Tile), With<Tile>>,
    mut player_query: Query<(Entity, &Transform, &mut PlayerController, &mut Velocity, &Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Velocity, &Health), (With<Enemy>, Without<Player>)>,
    water_query: Query<(Entity, &InWater)>,
) {
    let tile_size = 32.0;
    let half_tile = tile_size / 2.0;
    let water_detection_radius = tile_size * 0.8; // Larger detection area to prevent gaps
    
    // Check players in water
    for (player_entity, player_transform, mut controller, mut velocity, health) in player_query.iter_mut() {
        // Skip dead players
        if health.is_dead() {
            continue;
        }
        
        let player_pos = player_transform.translation.truncate();
        let mut in_water = false;
        let mut closest_water_distance = f32::MAX;
        
        // Find the closest water tile
        for (tile_transform, tile) in tile_query.iter() {
            if tile.tile_type == TileType::Water {
                let tile_pos = tile_transform.translation.truncate();
                let distance = player_pos.distance(tile_pos);
                
                if distance < water_detection_radius {
                    in_water = true;
                    closest_water_distance = closest_water_distance.min(distance);
                }
            }
        }
        
        let mut water_depth = 0.0;
        if in_water {
            // Calculate depth based on closest water tile, with smoother transition
            water_depth = ((water_detection_radius - closest_water_distance) / water_detection_radius).max(0.0);
            // Slow down movement in water (50% speed)
            velocity.0 *= 0.5;
        }
        
        // Add or update InWater component safely with smoothing
        if in_water {
            if let Ok((_, existing_water)) = water_query.get(player_entity) {
                // Only update if there's a significant change (reduces flickering)
                if (existing_water.depth - water_depth).abs() > 0.05 {
                    // Use try_insert to handle potential entity issues
                    if let Ok(mut entity_commands) = commands.get_entity(player_entity) {
                        entity_commands.insert(InWater { depth: water_depth });
                    }
                }
            } else {
                // Use try_insert to handle potential entity issues
                if let Ok(mut entity_commands) = commands.get_entity(player_entity) {
                    entity_commands.insert(InWater { depth: water_depth });
                }
            }
        } else if water_query.get(player_entity).is_ok() {
            // Use try_remove to handle potential entity issues
            if let Ok(mut entity_commands) = commands.get_entity(player_entity) {
                entity_commands.remove::<InWater>();
            }
        }
    }
    
    // Check enemies in water
    for (enemy_entity, enemy_transform, mut velocity, health) in enemy_query.iter_mut() {
        // Skip dead enemies
        if health.is_dead() {
            continue;
        }
        
        let enemy_pos = enemy_transform.translation.truncate();
        let mut in_water = false;
        let mut closest_water_distance = f32::MAX;
        
        // Find the closest water tile for enemies too
        for (tile_transform, tile) in tile_query.iter() {
            if tile.tile_type == TileType::Water {
                let tile_pos = tile_transform.translation.truncate();
                let distance = enemy_pos.distance(tile_pos);
                
                if distance < water_detection_radius {
                    in_water = true;
                    closest_water_distance = closest_water_distance.min(distance);
                }
            }
        }
        
        let mut water_depth = 0.0;
        if in_water {
            water_depth = ((water_detection_radius - closest_water_distance) / water_detection_radius).max(0.0);
            // Slow down enemies in water (30% speed)
            velocity.0 *= 0.3;
        }
        
        if in_water {
            if let Ok((_, existing_water)) = water_query.get(enemy_entity) {
                if (existing_water.depth - water_depth).abs() > 0.05 {
                    if let Ok(mut entity_commands) = commands.get_entity(enemy_entity) {
                        entity_commands.insert(InWater { depth: water_depth });
                    }
                }
            } else {
                if let Ok(mut entity_commands) = commands.get_entity(enemy_entity) {
                    entity_commands.insert(InWater { depth: water_depth });
                }
            }
        } else if water_query.get(enemy_entity).is_ok() {
            if let Ok(mut entity_commands) = commands.get_entity(enemy_entity) {
                entity_commands.remove::<InWater>();
            }
        }
    }
}
