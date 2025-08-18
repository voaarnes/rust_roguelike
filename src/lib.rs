pub mod prelude {
    pub use crate::animation::{
        sprite_sheet::{SpriteSheetAnimation, AnimationClip},
        AnimationPlugin,
    };
    pub use crate::audio::{AudioManager, AudioPlugin};
    pub use crate::tilemap::{
        tilemap::{Tile, TileType, Tilemap},
        TilemapPlugin,
    };
    pub use crate::entities::{
        player::Player,
        enemy::{Enemy, EnemyType},
        collectible::{Collectible, CollectibleType},
        EntitiesPlugin,
    };
}
