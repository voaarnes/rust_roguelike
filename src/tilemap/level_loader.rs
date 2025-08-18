// src/tilemap/level_loader.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub tileset: String,
    pub tile_size: u32,
    pub width: usize,
    pub height: usize,
    pub layers: Vec<LayerData>,
    pub entities: Vec<EntitySpawn>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerData {
    pub name: String,
    pub z_index: f32,
    pub tiles: Vec<Vec<u32>>, // 2D array of tile indices
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitySpawn {
    pub entity_type: String,
    pub x: f32,
    pub y: f32,
    pub properties: std::collections::HashMap<String, String>,
}

pub fn load_level_from_json(path: &str) -> Result<LevelData, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let level: LevelData = serde_json::from_str(&contents)?;
    Ok(level)
}

pub fn load_level_from_pyxel(path: &str) -> Result<LevelData, Box<dyn std::error::Error>> {
    // Parse Pyxel format (assuming it exports as CSV or similar)
    let contents = fs::read_to_string(path)?;
    let lines: Vec<&str> = contents.lines().collect();
    
    // Parse header
    let header: Vec<&str> = lines[0].split(',').collect();
    let width = header[0].parse::<usize>()?;
    let height = header[1].parse::<usize>()?;
    let tile_size = header[2].parse::<u32>()?;
    
    // Parse tiles
    let mut tiles = vec![vec![0u32; width]; height];
    for (y, line) in lines.iter().skip(1).take(height).enumerate() {
        let row: Vec<u32> = line.split(',')
            .map(|s| s.trim().parse::<u32>().unwrap_or(0))
            .collect();
        tiles[y] = row;
    }
    
    Ok(LevelData {
        name: "Level".to_string(),
        tileset: "sprites/tileset.png".to_string(),
        tile_size,
        width,
        height,
        layers: vec![LayerData {
            name: "main".to_string(),
            z_index: 0.0,
            tiles,
        }],
        entities: Vec::new(),
    })
}

// Example JSON level file: assets/levels/level1.json
/*
{
    "name": "Dungeon Level 1",
    "tileset": "sprites/dungeon_tileset.png",
    "tile_size": 32,
    "width": 20,
    "height": 15,
    "layers": [
        {
            "name": "background",
            "z_index": 0.0,
            "tiles": [
                [1, 1, 1, 1, 1, ...],
                [1, 0, 0, 0, 1, ...],
                ...
            ]
        },
        {
            "name": "collision",
            "z_index": 1.0,
            "tiles": [
                [17, 17, 17, 17, 17, ...],
                [17, 0, 0, 0, 17, ...],
                ...
            ]
        }
    ],
    "entities": [
        {
            "entity_type": "player_spawn",
            "x": 160.0,
            "y": 240.0,
            "properties": {}
        },
        {
            "entity_type": "enemy",
            "x": 320.0,
            "y": 240.0,
            "properties": {
                "type": "goblin",
                "health": "30"
            }
        }
    ]
}
*/

// src/tilemap/tileset_config.rs
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct TilesetConfig {
    pub tile_definitions: HashMap<u32, TileDefinition>,
}

#[derive(Clone)]
pub struct TileDefinition {
    pub name: String,
    pub tile_type: TileType,
    pub collision: CollisionType,
    pub animation_frames: Option<Vec<u32>>, // For animated tiles
    pub properties: HashMap<String, String>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Floor,
    Wall,
    Door,
    Chest,
    Hazard,
    Decoration,
    Interactive,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CollisionType {
    None,
    Solid,
    Platform,    // Can jump through from below
    Trigger,     // Triggers events
    Damage,      // Deals damage
}

impl Default for TilesetConfig {
    fn default() -> Self {
        let mut definitions = HashMap::new();
        
        // Define your tile types based on index
        definitions.insert(0, TileDefinition {
            name: "empty".to_string(),
            tile_type: TileType::Empty,
            collision: CollisionType::None,
            animation_frames: None,
            properties: HashMap::new(),
        });
        
        // Floor tiles (1-16)
        for i in 1..=16 {
            definitions.insert(i, TileDefinition {
                name: format!("floor_{}", i),
                tile_type: TileType::Floor,
                collision: CollisionType::None,
                animation_frames: None,
                properties: HashMap::new(),
            });
        }
        
        // Wall tiles (17-32)
        for i in 17..=32 {
            definitions.insert(i, TileDefinition {
                name: format!("wall_{}", i),
                tile_type: TileType::Wall,
                collision: CollisionType::Solid,
                animation_frames: None,
                properties: HashMap::new(),
            });
        }
        
        // Animated water tiles (45-48)
        definitions.insert(45, TileDefinition {
            name: "water".to_string(),
            tile_type: TileType::Hazard,
            collision: CollisionType::Trigger,
            animation_frames: Some(vec![45, 46, 47, 48]),
            properties: HashMap::from([
                ("damage".to_string(), "5".to_string()),
            ]),
        });
        
        Self { tile_definitions: definitions }
    }
}

// src/tilemap/tilemap_spawner.rs
use super::tileset_config::{TilesetConfig, CollisionType};
use super::level_loader::LevelData;
use bevy::prelude::*;

#[derive(Component)]
pub struct TilemapLayer {
    pub name: String,
    pub z_index: f32,
}

#[derive(Component)]
pub struct AnimatedTile {
    pub frames: Vec<u32>,
    pub current_frame: usize,
    pub timer: Timer,
}

pub fn spawn_level(
    commands: &mut Commands,
    level_data: &LevelData,
    asset_server: &Res<AssetServer>,
    tileset_config: &TilesetConfig,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let tileset_handle: Handle<Image> = asset_server.load(&level_data.tileset);
    
    // Calculate tileset dimensions (assuming square tiles and known tileset size)
    let tileset_columns = 16; // You might want to make this configurable
    let tileset_rows = 16;
    
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(level_data.tile_size, level_data.tile_size),
        tileset_columns,
        tileset_rows,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    
    for layer in &level_data.layers {
        for (y, row) in layer.tiles.iter().enumerate() {
            for (x, &tile_index) in row.iter().enumerate() {
                if tile_index == 0 {
                    continue; // Skip empty tiles
                }
                
                let world_pos = Vec3::new(
                    x as f32 * level_data.tile_size as f32,
                    (level_data.height - y - 1) as f32 * level_data.tile_size as f32,
                    layer.z_index,
                );
                
                let mut entity_commands = commands.spawn((
                    Sprite {
                        image: tileset_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle.clone(),
                            index: tile_index as usize,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    TilemapLayer {
                        name: layer.name.clone(),
                        z_index: layer.z_index,
                    },
                ));
                
                // Add collision if needed
                if let Some(tile_def) = tileset_config.tile_definitions.get(&tile_index) {
                    match tile_def.collision {
                        CollisionType::Solid => {
                            entity_commands.insert(Collider::Solid);
                        }
                        CollisionType::Platform => {
                            entity_commands.insert(Collider::Platform);
                        }
                        CollisionType::Trigger => {
                            entity_commands.insert(Collider::Trigger);
                        }
                        CollisionType::Damage => {
                            entity_commands.insert((
                                Collider::Trigger,
                                DamageZone {
                                    damage: tile_def.properties
                                        .get("damage")
                                        .and_then(|d| d.parse::<i32>().ok())
                                        .unwrap_or(1),
                                },
                            ));
                        }
                        _ => {}
                    }
                    
                    // Add animation if needed
                    if let Some(frames) = &tile_def.animation_frames {
                        entity_commands.insert(AnimatedTile {
                            frames: frames.clone(),
                            current_frame: 0,
                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                        });
                    }
                }
            }
        }
    }
    
    // Spawn entities defined in the level
    for entity_spawn in &level_data.entities {
        spawn_level_entity(commands, asset_server, entity_spawn);
    }
}

fn spawn_level_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_spawn: &super::level_loader::EntitySpawn,
) {
    use crate::entities::{enemy, collectible};
    
    match entity_spawn.entity_type.as_str() {
        "player_spawn" => {
            // Mark player spawn point
            commands.spawn((
                Transform::from_xyz(entity_spawn.x, entity_spawn.y, 0.0),
                PlayerSpawnPoint,
            ));
        }
        "enemy" => {
            let enemy_type = match entity_spawn.properties.get("type").map(|s| s.as_str()) {
                Some("goblin") => enemy::EnemyType::Goblin,
                Some("skeleton") => enemy::EnemyType::Skeleton,
                Some("orc") => enemy::EnemyType::Orc,
                _ => enemy::EnemyType::Goblin,
            };
            enemy::spawn_enemy(
                commands,
                asset_server,
                Vec3::new(entity_spawn.x, entity_spawn.y, 0.0),
                enemy_type,
            );
        }
        "coin" => {
            collectible::spawn_collectible(
                commands,
                asset_server,
                Vec3::new(entity_spawn.x, entity_spawn.y, 0.0),
                collectible::CollectibleType::Coin,
            );
        }
        _ => {}
    }
}

// Components for collision and game mechanics
#[derive(Component)]
pub enum Collider {
    Solid,
    Platform,
    Trigger,
}

#[derive(Component)]
pub struct DamageZone {
    pub damage: i32,
}

#[derive(Component)]
pub struct PlayerSpawnPoint;

// System to animate tiles
pub fn animate_tiles(
    time: Res<Time>,
    mut query: Query<(&mut AnimatedTile, &mut Sprite)>,
) {
    for (mut animated_tile, mut sprite) in query.iter_mut() {
        animated_tile.timer.tick(time.delta());
        
        if animated_tile.timer.just_finished() {
            animated_tile.current_frame = 
                (animated_tile.current_frame + 1) % animated_tile.frames.len();
            
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animated_tile.frames[animated_tile.current_frame] as usize;
            }
        }
    }
}

// Example Pyxel Studio export format (CSV):
// assets/levels/level1.csv
/*
20,15,32
17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17
17,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,17
17,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,17
17,1,0,0,33,0,0,0,0,45,45,0,0,0,0,33,0,0,1,17
17,1,0,0,0,0,0,0,0,45,45,0,0,0,0,0,0,0,1,17
17,1,0,0,0,0,17,17,17,17,17,17,17,0,0,0,0,0,1,17
17,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,17
17,1,0,41,41,41,0,0,0,0,0,0,0,41,41,41,0,0,1,17
17,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,17
17,1,0,0,0,0,0,37,37,37,37,0,0,0,0,0,0,0,1,17
17,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,17
17,1,0,0,0,17,17,17,0,0,0,17,17,17,0,0,0,0,1,17
17,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,17
17,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,17
17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17
*/

// Tileset Index Reference (for your Pyxel Studio):
/*
TILESET INDEX GUIDE:
0: Empty/Transparent
1-16: Floor variations
17-32: Wall variations
33-36: Doors
37-40: Chests
41-44: Spikes/Hazards
45-48: Water (animated)
49-52: Lava (animated)
53-60: Decorations (torches, barrels, etc.)
61-64: Interactive objects (switches, levers)
65-80: Character sprites (if in same tileset)
*/
