use super::tilemap::{Tile, TileType, Tilemap, TilemapConfig, MapSizePx};
use bevy::prelude::*;

pub fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<TilemapConfig>,
) {
    let level_data = r#"
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
"#;

    let tilemap = Tilemap::from_string(level_data);

    let tileset_handle: Handle<Image> = asset_server.load("sprites/tileset.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.tile_size as u32, config.tile_size as u32),
        config.tileset_columns as u32,
        config.tileset_rows as u32,
        None,
        None,
    );
    let layout_handle = atlas_layouts.add(layout);

    let map_w = tilemap.width as f32 * config.tile_size;
    let map_h = tilemap.height as f32 * config.tile_size;
    commands.insert_resource(MapSizePx { w: map_w, h: map_h });

    let origin_x = -map_w * 0.5 + config.tile_size * 0.5;
    let origin_y = -map_h * 0.5 + config.tile_size * 0.5;

    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            if let Some(tile_type) = tilemap.tiles[y][x] {
                let tile_index = get_tile_index(tile_type);

                let world_pos = Vec3::new(
                    origin_x + (x as f32) * config.tile_size,
                    origin_y + ((tilemap.height - y - 1) as f32) * config.tile_size,
                    0.0,
                );

                commands.spawn((
                    Sprite {
                        image: tileset_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle.clone(),
                            index: tile_index,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    Tile { tile_type, walkable: is_walkable(tile_type), tile_index },
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
