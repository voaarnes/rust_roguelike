use super::tilemap::{Tile, TileType, Tilemap, TilemapConfig};
use bevy::prelude::*;

pub fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Example level data - you can load this from a file
    let level_data = "
########################################
#......................................#
#.....###...###...###..................#
#.....#.....#.#...#.#..................#
#.....###...#.#...###..................#
#.....#.....#.#...#.#..................#
#.....#.....###...#.#..................#
#......................................#
#......................................#
#...........^^^^.......................#
#......................................#
#......................................#
#.....CCCC.............................#
#......................................#
#......................................#
#.....................~~~~~~~~~~~~~~~~~#
#.....................~~~~~~~~~~~~~~~~~#
#.....................~~~~~~~~~~~~~~~~~#
########################################
";

    let tilemap = Tilemap::from_string(level_data);
    let config = TilemapConfig::default();
    
    // Load the tileset texture
    let tileset_handle: Handle<Image> = asset_server.load("sprites/tileset.png");
    
    // Create texture atlas layout
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.tile_size as u32, config.tile_size as u32),
        config.tileset_columns as u32,
        config.tileset_rows as u32,
        None,
        None,
    );
    
    // Spawn tiles
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            if let Some(tile_type) = tilemap.tiles[y][x] {
                let tile_index = get_tile_index(tile_type);
                let world_pos = Vec3::new(
                    x as f32 * config.tile_size,
                    (tilemap.height - y - 1) as f32 * config.tile_size,
                    0.0,
                );
                
                commands.spawn((
                    Sprite {
                        image: tileset_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: Handle::weak_from_u128(0), // You'll need to properly handle this
                            index: tile_index,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    Tile {
                        tile_type,
                        walkable: is_walkable(tile_type),
                        tile_index,
                    },
                ));
            }
        }
    }
}

fn get_tile_index(tile_type: TileType) -> usize {
    match tile_type {
        TileType::Floor => 1,
        TileType::Wall => 17,
        TileType::Door => 33,
        TileType::Chest => 37,
        TileType::Spike => 41,
        TileType::Water => 45,
    }
}

fn is_walkable(tile_type: TileType) -> bool {
    match tile_type {
        TileType::Floor | TileType::Door => true,
        TileType::Wall | TileType::Chest | TileType::Spike | TileType::Water => false,
    }
}
