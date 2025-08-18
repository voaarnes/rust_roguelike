use bevy::prelude::*;

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

pub struct Tilemap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Option<TileType>>>,
}

impl Tilemap {

    pub fn from_string(map_string: &str) -> Self {
        // Drop empty/whitespace-only lines (e.g., the very first blank line)
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
                    '#' => Some(TileType::Wall),
                    '.' => Some(TileType::Floor),
                    'D' => Some(TileType::Door),
                    'C' => Some(TileType::Chest),
                    '^' => Some(TileType::Spike),
                    '~' => Some(TileType::Water),
                    _ => None,
                };
            }
        }

        Self { width, height, tiles }
    }
    
    pub fn from_indices(map_data: Vec<Vec<usize>>) -> Self {
        let height = map_data.len();
        let width = map_data.get(0).map_or(0, |row| row.len());
        
        let mut tiles = vec![vec![None; width]; height];
        
        for (y, row) in map_data.iter().enumerate() {
            for (x, &index) in row.iter().enumerate() {
                tiles[y][x] = match index {
                    0 => None,
                    1..=16 => Some(TileType::Floor),
                    17..=32 => Some(TileType::Wall),
                    33..=36 => Some(TileType::Door),
                    37..=40 => Some(TileType::Chest),
                    41..=44 => Some(TileType::Spike),
                    45..=48 => Some(TileType::Water),
                    _ => None,
                };
            }
        }
        
        Self { width, height, tiles }
    }
}
