use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TilemapConfig>()
            .add_systems(Startup, crate::world::level_loader::load_level);
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
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
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
