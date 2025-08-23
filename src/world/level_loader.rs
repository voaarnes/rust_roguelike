use bevy::prelude::*;
use crate::game::movement::Collider;
use crate::world::tilemap::{Tile, TileType, TilemapConfig, AnimatedTile};

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Interactive {
    pub interaction_type: InteractionType,
}

#[derive(Clone, Copy)]
pub enum InteractionType {
    Door,
    Chest,
    Portal,
}

pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<TilemapConfig>,
) {
    spawn_level(&mut commands, &asset_server, &mut texture_atlas_layouts, &config);
}

pub fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<TilemapConfig>,
) {
    spawn_level(&mut commands, &asset_server, &mut texture_atlas_layouts, &config);
}

fn spawn_level(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    config: &Res<TilemapConfig>,
) {
    let level_data = r#"
################################################
#..............................................#
#.....###...###...###.........####.............#
#.....#.....#.#...#.#.........#..#.............#
#.....###...#.#...###.........#..#.............#
#.....#.....#.#...#.#.........####.............#
#.....#.....###...#.#..........................#
#..............................................#
#...####....####....####....####....####.......#
#...#..#....#..#....#..#....#..#....#..#.......#
#...#..######..######..######..######..#.......#
#...#..........................................#
#...####....####....####....####....####.......#
#..............................................#
#.....CCCC......................CCCC...........#
#..............................................#
#...################....################.......#
#..........................................^^^^#
#..........................................^^^^#
#..............................................#
#...~~~~~..~~~~~..~~~~~..~~~~~..~~~~~..~~~~~...#
#...~~~~~..~~~~~..~~~~~..~~~~~..~~~~~..~~~~~...#
#..............................................#
#..####....####....####....####....####....###.#
#..#..#....#..#....#..#....#..#....#..#....#...#
#..#..######..######..######..######..######...#
#..............................................#
#......^^^^....................................#
#......^^^^....................................#
################################################
"#;

    let lines: Vec<&str> = level_data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    let tileset_handle: Handle<Image> = asset_server.load("sprites/tileset_16x16_32px.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.tile_size as u32, config.tile_size as u32),
        config.tileset_columns as u32,
        config.tileset_rows as u32,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    
    let map_w = width as f32 * config.tile_size;
    let map_h = height as f32 * config.tile_size;
    let origin_x = -map_w * 0.5 + config.tile_size * 0.5;
    let origin_y = -map_h * 0.5 + config.tile_size * 0.5;
    
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let tile_type = match ch {
                '#' => Some(TileType::Wall),
                '.' => Some(TileType::Floor),
                'D' => Some(TileType::Door),
                'C' => Some(TileType::Chest),
                '^' => Some(TileType::Spike),
                '~' => Some(TileType::Water),
                _ => None,
            };

            if let Some(tile_type) = tile_type {
                let tile = Tile::new(tile_type);
                let tile_walkable = tile.walkable;  // Save walkable state before move

                let world_pos = Vec3::new(
                    origin_x + x as f32 * config.tile_size,
                    origin_y + (height as f32 - 1.0 - y as f32) * config.tile_size,
                    0.0,
                );

                let mut entity_commands = commands.spawn((
                    Sprite {
                        image: tileset_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle.clone(),
                            index: tile.tile_index,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    tile,  // tile is moved here
                ));

                // Use the saved walkable state instead of tile.walkable
                if !tile_walkable {
                    entity_commands.insert(Wall);
                    entity_commands.insert(Collider { size: Vec2::splat(config.tile_size) });
                }
                // Add AnimatedTile for water, lava, and portals
                match tile_type {
                    TileType::Water => {
                        entity_commands.insert(AnimatedTile {
                            frames: vec![45, 46, 47, 48],
                            current_frame: 0,
                            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                        });
                    }
                    TileType::Lava => {
                        entity_commands.insert(AnimatedTile {
                            frames: vec![49, 50, 51, 52],
                            current_frame: 0,
                            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                        });
                    }
                    TileType::Portal => {
                        entity_commands.insert(AnimatedTile {
                            frames: vec![53, 54, 55, 56],
                            current_frame: 0,
                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                        });
                    }
                    _ => {}
                }
                
                // Add wall collider for non-walkable tiles
                if !tile.walkable {
                    entity_commands.insert(Wall);
                    entity_commands.insert(Collider { size: Vec2::splat(config.tile_size) });
                }
                
                // Add interactive components
                match tile_type {
                    TileType::Door => {
                        entity_commands.insert(Interactive {
                            interaction_type: InteractionType::Door,
                        });
                    }
                    TileType::Chest => {
                        entity_commands.insert(Interactive {
                            interaction_type: InteractionType::Chest,
                        });
                    }
                    TileType::Portal => {
                        entity_commands.insert(Interactive {
                            interaction_type: InteractionType::Portal,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn cleanup_level(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
) {
    for e in tiles.iter() {
        commands.entity(e).despawn();
    }
}

pub fn despawn_level(
    commands: Commands,
    tiles: Query<Entity, With<Tile>>,
) {
    cleanup_level(commands, tiles);
}
