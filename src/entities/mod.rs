// Bridge module to maintain compatibility with old code
pub use crate::game::player;
pub use crate::game::enemy;

pub mod collectible {
    pub use crate::game::collectible::*;
}

pub mod powerup {
    #[derive(Component, Clone)]
    pub struct PowerUpSlots {
        pub slots: Vec<Option<PowerUpType>>,
        pub max_slots: usize,
    }
    
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PowerUpType {
        SpeedBoost,
        DamageBoost,
        HealthBoost,
        ShieldBoost,
    }
    
    impl PowerUpSlots {
        pub fn new(max_slots: usize) -> Self {
            Self {
                slots: vec![None; max_slots],
                max_slots,
            }
        }
    }
    
    use bevy::prelude::*;
}
