// Bridge module to maintain compatibility with old code
pub use crate::game::player;
pub use crate::game::enemy;

pub mod collectible {
    pub use crate::game::collectible::*;
}



// src/entities/mod.rs
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
        
        // Fixed: Now returns Option<PowerUpType> for the dropped item
        pub fn add_powerup(&mut self, powerup: PowerUpType) -> Option<PowerUpType> {
            // Find first empty slot
            for slot in &mut self.slots {
                if slot.is_none() {
                    *slot = Some(powerup);
                    return None; // Nothing was dropped
                }
            }
            
            // If no empty slot, replace the oldest (first) one
            let dropped = self.slots[0]; // Save what we're dropping
            
            // Shift all elements left
            for i in 0..self.slots.len() - 1 {
                self.slots[i] = self.slots[i + 1];
            }
            
            // Add new powerup at the end
            let last_idx = self.slots.len() - 1;
            self.slots[last_idx] = Some(powerup);
            
            dropped // Return what was dropped
        }
        
        // Add missing methods
        pub fn get_head_fruit(&self) -> Option<PowerUpType> {
            // Head is the newest (last added)
            for i in (0..self.slots.len()).rev() {
                if self.slots[i].is_some() {
                    return self.slots[i];
                }
            }
            None
        }
        
        pub fn get_legs_fruit(&self) -> Option<PowerUpType> {
            // Legs is the oldest (first non-empty)
            for i in 0..self.slots.len() {
                if self.slots[i].is_some() {
                    return self.slots[i];
                }
            }
            None
        }
    }
}
