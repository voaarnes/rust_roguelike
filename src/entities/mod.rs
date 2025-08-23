// Bridge module to maintain compatibility with old code
pub use crate::game::player;
pub use crate::game::enemy;

pub mod collectible {
    pub use crate::game::collectible::*;
}


pub mod powerup;

