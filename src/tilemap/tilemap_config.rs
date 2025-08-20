// Separate config file for easier tile mapping management
use bevy::prelude::*;
use std::collections::HashMap;
use super::tilemap::{TileDefinition, TileType, LayerType};

pub fn create_pyxel_edit_config() -> HashMap<char, TileDefinition> {
    let mut definitions = HashMap::new();
    
    // PYXEL EDIT TILE MAPPING
    // Assuming your tileset is 16x16 tiles (256 total)
    // Index = row * 16 + column (0-indexed)
    
    // === FLOORS (Row 0, indices 0-15) ===
    definitions.insert('.', TileDefinition {
        tile_type: TileType::Floor,
        base_index: 1,  // Column 1, Row 0
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Background,
    });
    
    definitions.insert('g', TileDefinition {
        tile_type: TileType::Grass,
        base_index: 2,  // Column 2, Row 0
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Background,
    });
    
    definitions.insert('s', TileDefinition {
        tile_type: TileType::Stone,
        base_index: 3,  // Column 3, Row 0
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Background,
    });
    
    definitions.insert('w', TileDefinition {
        tile_type: TileType::Wood,
        base_index: 4,  // Column 4, Row 0
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Background,
    });
    
    // === WALLS (Row 1, indices 16-31) ===
    definitions.insert('#', TileDefinition {
        tile_type: TileType::Wall,
        base_index: 17,  // Column 1, Row 1 (1*16 + 1)
        walkable: false,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Collision,
    });
    
    // You can add wall variants
    definitions.insert('W', TileDefinition {  // Alternative wall
        tile_type: TileType::Wall,
        base_index: 18,  // Column 2, Row 1
        walkable: false,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Collision,
    });
    
    // === DOORS (Row 2, indices 32-47) ===
    definitions.insert('D', TileDefinition {
        tile_type: TileType::Door,
        base_index: 33,  // Column 1, Row 2 (2*16 + 1)
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Collision,
    });
    
    definitions.insert('C', TileDefinition {
        tile_type: TileType::Chest,
        base_index: 37,  // Column 5, Row 2 (2*16 + 5)
        walkable: false,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Decoration,
    });
    
    // === HAZARDS (Row 2-3, indices 40-63) ===
    definitions.insert('^', TileDefinition {
        tile_type: TileType::Spike,
        base_index: 41,  // Column 9, Row 2 (2*16 + 9)
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Decoration,
    });
    
    // === ANIMATED WATER (Row 2-3, use 4 consecutive tiles) ===
    definitions.insert('~', TileDefinition {
        tile_type: TileType::Water,
        base_index: 45,  // Column 13, Row 2 (2*16 + 13)
        walkable: false,
        animated: true,
        animation_frames: vec![45, 46, 47, 48],  // 4 frames of animation
        animation_speed: 0.5,
        layer: LayerType::Background,
    });
    
    // === ANIMATED LAVA (Row 3) ===
    definitions.insert('L', TileDefinition {
        tile_type: TileType::Lava,
        base_index: 49,  // Column 1, Row 3 (3*16 + 1)
        walkable: false,
        animated: true,
        animation_frames: vec![49, 50, 51, 52],  // 4 frames
        animation_speed: 0.3,
        layer: LayerType::Background,
    });
    
    // === PORTAL (Row 3) ===
    definitions.insert('P', TileDefinition {
        tile_type: TileType::Portal,
        base_index: 53,  // Column 5, Row 3 (3*16 + 5)
        walkable: true,
        animated: true,
        animation_frames: vec![53, 54, 55, 56],
        animation_speed: 0.2,
        layer: LayerType::Decoration,
    });
    
    // Add more mappings as needed...
    
    definitions
}

// Helper function to calculate tile index from row/column
pub fn tile_index(row: usize, column: usize) -> usize {
    row * 16 + column
}

// Custom mapping for your specific tileset
pub fn create_custom_mapping() -> HashMap<char, TileDefinition> {
    let mut definitions = HashMap::new();
    
    // Example: If you want to map specific Pyxel Edit tiles
    // Just change the indices to match your tileset layout
    
    definitions.insert('a', TileDefinition {
        tile_type: TileType::Floor,
        base_index: tile_index(0, 0),  // Top-left tile
        walkable: true,
        animated: false,
        animation_frames: vec![],
        animation_speed: 0.0,
        layer: LayerType::Background,
    });
    
    // Add your custom mappings here...
    
    definitions
}
