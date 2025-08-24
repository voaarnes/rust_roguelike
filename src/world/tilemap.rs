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
            // Check if entity still exists before trying to modify it
            if commands.get_entity(player_entity).is_ok() {
                commands.entity(player_entity).insert(OnSpikes);
            }
        } else if !on_spikes && spike_query.contains(player_entity) {
            // Only remove if entity still exists
            if commands.get_entity(player_entity).is_ok() {
                commands.entity(player_entity).remove::<OnSpikes>();
            }
        }
    }
    
    // Check enemies on spikes
    for (enemy_entity, enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
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
            // Check if entity still exists before trying to modify it
            if commands.get_entity(enemy_entity).is_ok() {
                commands.entity(enemy_entity).insert(OnSpikes);
            }
        } else if !on_spikes && spike_query.contains(enemy_entity) {
            // Only remove if entity still exists
            if commands.get_entity(enemy_entity).is_ok() {
                commands.entity(enemy_entity).remove::<OnSpikes>();
            }
        }
    }
}

// System to handle water effects (slower movement, visual depth)
fn apply_water_effects(
    mut commands: Commands,
    tile_query: Query<(&Transform, &Tile), With<Tile>>,
    mut player_query: Query<(Entity, &Transform, &mut PlayerController, &mut Velocity), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Velocity), (With<Enemy>, Without<Player>)>,
    water_query: Query<(Entity, &InWater)>,
) {
    let tile_size = 32.0;
    let half_tile = tile_size / 2.0;
    
    // Check players in water
    for (player_entity, player_transform, mut controller, mut velocity) in player_query.iter_mut() {
        let player_pos = player_transform.translation.truncate();
        let mut in_water = false;
        let mut water_depth = 0.0;
        
        for (tile_transform, tile) in tile_query.iter() {
            if tile.tile_type == TileType::Water {
                let tile_pos = tile_transform.translation.truncate();
                let distance = player_pos.distance(tile_pos);
                
                if distance < half_tile {
                    in_water = true;
                    water_depth = (half_tile - distance) / half_tile; // 0.0 to 1.0
                    
                    // Slow down movement in water (50% speed)
                    velocity.0 *= 0.5;
                    break;
                }
            }
        }
        
        // Add or update InWater component safely
        if in_water {
            // Check if entity still exists before trying to modify it
            if commands.get_entity(player_entity).is_ok() {
                if let Ok((_, existing_water)) = water_query.get(player_entity) {
                    if (existing_water.depth - water_depth).abs() > 0.1 {
                        commands.entity(player_entity).insert(InWater { depth: water_depth });
                    }
                } else {
                    commands.entity(player_entity).insert(InWater { depth: water_depth });
                }
            }
        } else if water_query.get(player_entity).is_ok() {
            // Only remove if entity still exists
            if commands.get_entity(player_entity).is_ok() {
                commands.entity(player_entity).remove::<InWater>();
            }
        }
    }
    
    // Check enemies in water
    for (enemy_entity, enemy_transform, mut velocity) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation.truncate();
        let mut in_water = false;
        let mut water_depth = 0.0;
        
        for (tile_transform, tile) in tile_query.iter() {
            if tile.tile_type == TileType::Water {
                let tile_pos = tile_transform.translation.truncate();
                let distance = enemy_pos.distance(tile_pos);
                
                if distance < half_tile {
                    in_water = true;
                    water_depth = (half_tile - distance) / half_tile;
                    
                    // Slow down enemies in water (30% speed)
                    velocity.0 *= 0.3;
                    break;
                }
            }
        }
        
        if in_water {
            // Check if entity still exists before trying to modify it
            if commands.get_entity(enemy_entity).is_ok() {
                if let Ok((_, existing_water)) = water_query.get(enemy_entity) {
                    if (existing_water.depth - water_depth).abs() > 0.1 {
                        commands.entity(enemy_entity).insert(InWater { depth: water_depth });
                    }
                } else {
                    commands.entity(enemy_entity).insert(InWater { depth: water_depth });
                }
            }
        } else if water_query.get(enemy_entity).is_ok() {
            // Only remove if entity still exists
            if commands.get_entity(enemy_entity).is_ok() {
                commands.entity(enemy_entity).remove::<InWater>();
            }
        }
    }
}
