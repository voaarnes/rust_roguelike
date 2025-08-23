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
pub struct AnimatedTile {
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: Timer,
}

#[derive(Component)]
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
            TileType::Wall | TileType::Chest | TileType::Water | TileType::Lava => false,
            TileType::Spike | TileType::Portal => true, // Walkable but may have effects
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
