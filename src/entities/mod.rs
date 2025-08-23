// Bridge module to maintain compatibility with old code
pub use crate::game::player;
pub use crate::game::enemy;

pub mod collectible {
    pub use crate::game::collectible::*;
}

pub mod powerup {
    use bevy::prelude::*;
    
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
        
        pub fn get_slots_as_vec(&self) -> Vec<Option<PowerUpType>> {
            self.slots.clone()
        }
        
        pub fn add_powerup(&mut self, powerup: PowerUpType) -> bool {
            // Find first empty slot
            for slot in &mut self.slots {
                if slot.is_none() {
                    *slot = Some(powerup);
                    return true;
                }
            }
            
            // If no empty slot, replace the oldest (first) one
            if !self.slots.is_empty() {
                // Shift all elements left and add new one at the end
                for i in 0..self.slots.len() - 1 {
                    self.slots[i] = self.slots[i + 1];
                }
                self.slots[self.slots.len() - 1] = Some(powerup);
                return true;
            }
            
            false
        }
    }
}
