use bevy::prelude::*;
use crate::game::movement::Collider;
use crate::world::tilemap::{Tile, TileType, TilemapConfig};

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
#...####....####....####....####....####......#
#...#..#....#..#....#..#....#..#....#..#......#
#...#..######..######..######..######..#......#
#...#..........................................#
#...####....####....####....####....####......#
#..............................................#
#.....CCCC......................CCCC...........#
#..............................................#
#...################....################.......#
#..........................................^^^^#
#..........................................^^^^#
#..............................................#
#...~~~~~..~~~~~..~~~~~..~~~~~..~~~~~..~~~~~..#
#...~~~~~..~~~~~..~~~~~..~~~~~..~~~~~..~~~~~..#
#..............................................#
#..####....####....####....####....####....####
#..#..#....#..#....#..#....#..#....#..#....#..#
#..#..######..######..######..######..######..#
#..............................................#
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
    
    let tileset_handle: Handle<Image> = asset_server.load("sprites/tileset.png");
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
                let world_pos = Vec3::new(
                    origin_x + (x as f32) * config.tile_size,
                    origin_y + ((height - y - 1) as f32) * config.tile_size,
                    0.0,
                );
                
                let tile_index = get_tile_index(tile_type);
                
                let mut entity_commands = commands.spawn((
                    Sprite {
                        image: tileset_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle.clone(),
                            index: tile_index,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    Tile {
                        tile_type,
                        walkable: is_walkable(tile_type),
                    },
                ));
                
                // Add collision for walls
                if tile_type == TileType::Wall {
                    entity_commands.insert((
                        Collider { size: Vec2::splat(config.tile_size) },
                        Wall,
                    ));
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
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
) {
    cleanup_level(commands, tiles);
}

fn get_tile_index(tile_type: TileType) -> usize {
    match tile_type {
        TileType::Floor => 1,
        TileType::Wall => 17,
        TileType::Door => 33,
        TileType::Chest => 37,
        TileType::Spike => 41,
        TileType::Water => 45,
        TileType::Portal => 49,
        _ => 0,
    }
}

fn is_walkable(tile_type: TileType) -> bool {
    matches!(tile_type, TileType::Floor | TileType::Door)
}
