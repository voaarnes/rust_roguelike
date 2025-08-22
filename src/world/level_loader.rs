use bevy::prelude::*;
use bevy::math::UVec2;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};
use crate::game::movement::Collider; // reuse your existing Collider

use super::tilemap::{
    Tilemap, TilemapConfig, Tile, AnimatedTile, TilemapLayer,
    DamageZone, Interactive, InteractionType,
    MapSizePx, TileType, LayerType,
};

pub fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<TilemapConfig>,
) {
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

    let tileset: Handle<Image> = asset_server.load("sprites/tileset_16x16_32px_hd.png");
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
            if let Some(ch) = tilemap.tiles[y][x] {
                if let Some(def) = config.tile_definitions.get(&ch) {
                    let pos = Vec3::new(
                        origin_x + (x as f32) * config.tile_size,
                        origin_y + ((tilemap.height - y - 1) as f32) * config.tile_size,
                        layer_z(def.layer),
                    );

                    let mut e = commands.spawn((
                        Sprite {
                            image: tileset.clone(),
                            texture_atlas: Some(TextureAtlas { layout: layout_handle.clone(), index: def.base_index }),
                            ..default()
                        },
                        Transform::from_translation(pos),
                        Tile { tile_type: def.tile_type, walkable: def.walkable, tile_index: def.base_index, layer: def.layer },
                        TilemapLayer { layer: def.layer },
                    ));

                    if def.animated {
                        e.insert(AnimatedTile {
                            frames: def.animation_frames.clone(),
                            current_frame: 0,
                            timer: Timer::from_seconds(def.animation_speed, TimerMode::Repeating),
                        });
                    }

                    if !def.walkable && matches!(def.tile_type, TileType::Wall) {
                        e.insert(Collider);
                    }

                    match def.tile_type {
                        TileType::Spike => { e.insert(DamageZone { damage: 10 }); }
                        TileType::Lava  => { e.insert(DamageZone { damage: 20 }); }
                        _ => {}
                    }

                    match def.tile_type {
                        TileType::Door   => e.insert(Interactive { interaction_type: InteractionType::Door }),
                        TileType::Chest  => e.insert(Interactive { interaction_type: InteractionType::Chest }),
                        TileType::Portal => e.insert(Interactive { interaction_type: InteractionType::Portal }),
                        _ => {}
                    }
                }
            }
        }
    }

    info!("Level loaded: {}x{} tiles", tilemap.width, tilemap.height);
}

fn layer_z(layer: LayerType) -> f32 {
    match layer {
        LayerType::Background => 0.0,
        LayerType::Collision  => 1.0,
        LayerType::Decoration => 2.0,
        LayerType::Overlay    => 3.0,
    }
}

pub fn despawn_level(
    mut commands: Commands,
    q: Query<Entity, Or<(With<Tile>, With<TilemapLayer>, With<AnimatedTile>)>>,
) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
