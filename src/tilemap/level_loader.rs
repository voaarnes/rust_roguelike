use super::tilemap::{Tilemap, TilemapConfig, Tile, AnimatedTile, TilemapLayer, Collider, DamageZone, Interactive, InteractionType, MapSizePx, TileType};
use bevy::prelude::*;

pub fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<TilemapConfig>,
) {
    // Enhanced test level with various tile types
    let level_data = r#"
########################################
#......................................#
#..gggggg..###...###...###.............#
#..gggggg..#.....#.#...#.#.............#
#..gggggg..###...#.#...###.............#
#..........#.....#.#...#.#.............#
#..........#.....###...#.#.............#
#......................................#
#...wwwwww.............................#
#...wwwwww..^^^^.......................#
#...wwwwww.............................#
#..................########............#
#.....CCCC.........ssssssss............#
#..................ssssssss............#
#..................ssssssss............#
#...........P..................~~~~~~~~#
#..............................~~~~~~~~#
#..............................~~~~~~~~#
#...LLLLLLLLLLLLLLLLLL.........~~~~~~~~#
########################################
"#;

    let tilemap = Tilemap::from_string(level_data);

    // Load tileset and create atlas layout
    let tileset_handle: Handle<Image> = asset_server.load("sprites/tileset_16x16_32px_hd.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.tile_size as u32, config.tile_size as u32),
        config.tileset_columns as u32,
        config.tileset_rows as u32,
        None,
        None,
    );
    let layout_handle = atlas_layouts.add(layout);

    // Calculate map size and centered origin
    let map_w = tilemap.width as f32 * config.tile_size;
    let map_h = tilemap.height as f32 * config.tile_size;
    commands.insert_resource(MapSizePx { w: map_w, h: map_h });

    let origin_x = -map_w * 0.5 + config.tile_size * 0.5;
    let origin_y = -map_h * 0.5 + config.tile_size * 0.5;

    // Spawn tiles
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            if let Some(tile_char) = tilemap.tiles[y][x] {
                if let Some(tile_def) = config.tile_definitions.get(&tile_char) {
                    let world_pos = Vec3::new(
                        origin_x + (x as f32) * config.tile_size,
                        origin_y + ((tilemap.height - y - 1) as f32) * config.tile_size,
                        get_layer_z(tile_def.layer),
                    );

                    let mut entity_builder = commands.spawn((
                        Sprite {
                            image: tileset_handle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: layout_handle.clone(),
                                index: tile_def.base_index,
                            }),
                            ..default()
                        },
                        Transform::from_translation(world_pos),
                        Tile {
                            tile_type: tile_def.tile_type,
                            walkable: tile_def.walkable,
                            tile_index: tile_def.base_index,
                            layer: tile_def.layer,
                        },
                        TilemapLayer {
                            layer: tile_def.layer,
                        },
                    ));

                    // Add animated tile component if needed
                    if tile_def.animated {
                        entity_builder.insert(AnimatedTile {
                            frames: tile_def.animation_frames.clone(),
                            current_frame: 0,
                            timer: Timer::from_seconds(tile_def.animation_speed, TimerMode::Repeating),
                        });
                    }

                    // Add collision component for walls and obstacles
                    if !tile_def.walkable && tile_def.tile_type == TileType::Wall {
                        entity_builder.insert(Collider);
                    }

                    // Add damage zone for hazards
                    match tile_def.tile_type {
                        TileType::Spike => {
                            entity_builder.insert(DamageZone { damage: 10 });
                        },
                        TileType::Lava => {
                            entity_builder.insert(DamageZone { damage: 20 });
                        },
                        _ => {}
                    }

                    // Add interactive component for special tiles
                    match tile_def.tile_type {
                        TileType::Door => {
                            entity_builder.insert(Interactive {
                                interaction_type: InteractionType::Door,
                            });
                        },
                        TileType::Chest => {
                            entity_builder.insert(Interactive {
                                interaction_type: InteractionType::Chest,
                            });
                        },
                        TileType::Portal => {
                            entity_builder.insert(Interactive {
                                interaction_type: InteractionType::Portal,
                            });
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    info!("Level loaded: {}x{} tiles", tilemap.width, tilemap.height);
}

fn get_layer_z(layer: super::tilemap::LayerType) -> f32 {
    use super::tilemap::LayerType;
    match layer {
        LayerType::Background => 0.0,
        LayerType::Collision => 1.0,
        LayerType::Decoration => 2.0,
        LayerType::Overlay => 3.0,
    }
}

// Support for loading from JSON files
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub tileset: String,
    pub tile_size: u32,
    pub width: usize,
    pub height: usize,
    pub data: String, // The map string
    pub spawn_points: Vec<SpawnPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpawnPoint {
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

pub fn spawn_level_from_data(
    commands: &mut Commands,
    level_data: &LevelData,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    config: &Res<TilemapConfig>,
) {
    let tilemap = Tilemap::from_string(&level_data.data);
    
    let tileset_handle: Handle<Image> = asset_server.load(&level_data.tileset);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(level_data.tile_size, level_data.tile_size),
        config.tileset_columns as u32,
        config.tileset_rows as u32,
        None,
        None,
    );
    let layout_handle = atlas_layouts.add(layout);

    // Similar tile spawning logic as above...
    // (Implementation details same as load_test_level but using level_data)
    
    // Spawn entities from spawn points
    for spawn_point in &level_data.spawn_points {
        spawn_entity_from_point(commands, asset_server, spawn_point);
    }
}

fn spawn_entity_from_point(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    spawn_point: &SpawnPoint,
) {
    match spawn_point.entity_type.as_str() {
        "player_spawn" => {
            commands.spawn((
                Transform::from_xyz(spawn_point.x, spawn_point.y, 10.0),
                PlayerSpawnPoint,
            ));
        },
        // Add more entity types as needed
        _ => {}
    }
}

#[derive(Component)]
pub struct PlayerSpawnPoint;
