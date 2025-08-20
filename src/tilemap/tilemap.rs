use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
    pub tile_index: usize,
    pub layer: LayerType,
}

#[derive(Component)]
pub struct AnimatedTile {
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: Timer,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Empty,
    Floor,
    Wall,
    Door,
    Chest,
    Spike,
    Water,
    Lava,
    Grass,
    Stone,
    Wood,
    Portal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LayerType {
    Background,
    Collision,
    Decoration,
    Overlay,
}

#[derive(Resource, Clone, Copy)]
pub struct MapSizePx {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct TilemapConfig {
    pub tile_size: f32,
    pub tileset_columns: usize,
    pub tileset_rows: usize,
    pub tile_definitions: HashMap<char, TileDefinition>,
}

#[derive(Clone)]
pub struct TileDefinition {
    pub tile_type: TileType,
    pub base_index: usize,
    pub walkable: bool,
    pub animated: bool,
    pub animation_frames: Vec<usize>,
    pub animation_speed: f32,
    pub layer: LayerType,
}

impl Default for TilemapConfig {
    fn default() -> Self {
        let mut definitions = HashMap::new();
        
        // Floor tiles
        definitions.insert('.', TileDefinition {
            tile_type: TileType::Floor,
            base_index: 1,
            walkable: true,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Background,
        });
        
        // Wall tiles
        definitions.insert('#', TileDefinition {
            tile_type: TileType::Wall,
            base_index: 17,
            walkable: false,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Collision,
        });
        
        // Door
        definitions.insert('D', TileDefinition {
            tile_type: TileType::Door,
            base_index: 33,
            walkable: true,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Collision,
        });
        
        // Chest
        definitions.insert('C', TileDefinition {
            tile_type: TileType::Chest,
            base_index: 37,
            walkable: false,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Decoration,
        });
        
        // Spikes
        definitions.insert('^', TileDefinition {
            tile_type: TileType::Spike,
            base_index: 41,
            walkable: true, // Can walk on but takes damage
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Decoration,
        });
        
        // Animated Water
        definitions.insert('~', TileDefinition {
            tile_type: TileType::Water,
            base_index: 45,
            walkable: false,
            animated: true,
            animation_frames: vec![45, 46, 47, 48],
            animation_speed: 0.5, // seconds per frame
            layer: LayerType::Background,
        });
        
        // Animated Lava
        definitions.insert('L', TileDefinition {
            tile_type: TileType::Lava,
            base_index: 49,
            walkable: false,
            animated: true,
            animation_frames: vec![49, 50, 51, 52],
            animation_speed: 0.3,
            layer: LayerType::Background,
        });
        
        // Grass
        definitions.insert('g', TileDefinition {
            tile_type: TileType::Grass,
            base_index: 2,
            walkable: true,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Background,
        });
        
        // Stone floor
        definitions.insert('s', TileDefinition {
            tile_type: TileType::Stone,
            base_index: 3,
            walkable: true,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Background,
        });
        
        // Wood floor
        definitions.insert('w', TileDefinition {
            tile_type: TileType::Wood,
            base_index: 4,
            walkable: true,
            animated: false,
            animation_frames: vec![],
            animation_speed: 0.0,
            layer: LayerType::Background,
        });
        
        // Portal (animated)
        definitions.insert('P', TileDefinition {
            tile_type: TileType::Portal,
            base_index: 53,
            walkable: true,
            animated: true,
            animation_frames: vec![53, 54, 55, 56],
            animation_speed: 0.2,
            layer: LayerType::Decoration,
        });
        
        Self {
            tile_size: 32.0,
            tileset_columns: 16,
            tileset_rows: 16,
            tile_definitions: definitions,
        }
    }
}

pub struct Tilemap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Option<char>>>,
}

impl Tilemap {
    pub fn from_string(map_string: &str) -> Self {
        let lines: Vec<&str> = map_string
            .lines()
            .map(|l| l.trim_end())
            .filter(|l| !l.is_empty())
            .collect();

        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let mut tiles = vec![vec![None; width]; height];

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if x >= width { break; }
                tiles[y][x] = match ch {
                    ' ' => None,
                    c => Some(c),
                };
            }
        }

        Self { width, height, tiles }
    }
}

#[derive(Component)]
pub struct TilemapLayer {
    pub layer: LayerType,
}

// Component to mark collision tiles
#[derive(Component)]
pub struct Collider;

// Component for damage zones
#[derive(Component)]
pub struct DamageZone {
    pub damage: i32,
}

// Component for interactive tiles
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
