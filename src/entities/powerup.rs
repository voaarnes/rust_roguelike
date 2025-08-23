// src/entities/powerup.rs

use bevy::prelude::*;
use std::collections::VecDeque;

/// Component that manages power-up slots for entities (mainly player)
/// Uses a deque structure where newest items are at the front
#[derive(Component, Clone)]
pub struct PowerUpSlots {
    pub slots: VecDeque<FruitSlot>,
    pub max_slots: usize,
}

/// Represents a fruit slot that contains both visual and functional data
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FruitSlot {
    pub fruit_type: u8,           // 0-6 for different fruits (visual)
    pub powerup: PowerUpType,     // The actual power-up effect
}

/// Available power-up types that fruits can provide
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerUpType {
    SpeedBoost,      // Strawberry/Pear
    DamageBoost,     // Mango/Pineapple  
    HealthBoost,     // Apple/Carrot
    ShieldBoost,     // Coconut
}

impl PowerUpSlots {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: VecDeque::with_capacity(max_slots),
            max_slots,
        }
    }
    
    /// Add a fruit power-up to the slots, maintaining visual fruit type info
    pub fn add_fruit(&mut self, fruit_type: u8, powerup: PowerUpType) -> Option<FruitSlot> {
        let fruit_slot = FruitSlot { fruit_type, powerup };
        
        let dropped = if self.slots.len() >= self.max_slots {
            self.slots.pop_back() // Remove oldest (back of deque)
        } else {
            None
        };
        
        self.slots.push_front(fruit_slot); // Add newest to front
        
        dropped
    }
    
    /// Legacy method for backward compatibility - use add_fruit instead
    #[deprecated(note = "Use add_fruit instead to maintain fruit visuals")]
    pub fn add_powerup(&mut self, powerup: PowerUpType) -> Option<PowerUpType> {
        // Default to first fruit of each type for backward compatibility
        let fruit_type = match powerup {
            PowerUpType::SpeedBoost => 0,   // Strawberry
            PowerUpType::DamageBoost => 2,  // Mango
            PowerUpType::HealthBoost => 4,  // Apple
            PowerUpType::ShieldBoost => 6,  // Coconut
        };
        
        self.add_fruit(fruit_type, powerup).map(|slot| slot.powerup)
    }
    
    /// Get power-up slots as a vector for UI systems
    pub fn get_slots_as_vec(&self) -> Vec<Option<PowerUpType>> {
        let mut vec = Vec::with_capacity(self.max_slots);
        for i in 0..self.max_slots {
            if i < self.slots.len() {
                vec.push(Some(self.slots[i].powerup));
            } else {
                vec.push(None);
            }
        }
        vec
    }
    
    /// Get the newest fruit (head) - visual for player head
    pub fn get_head_fruit(&self) -> Option<u8> {
        self.slots.get(0).map(|slot| slot.fruit_type)
    }
    
    /// Get the middle fruit (torso) - visual for player torso
    pub fn get_torso_fruit(&self) -> Option<u8> {
        self.slots.get(1).map(|slot| slot.fruit_type)
    }
    
    /// Get the oldest fruit (legs) - visual for player legs
    pub fn get_legs_fruit(&self) -> Option<u8> {
        self.slots.get(2).map(|slot| slot.fruit_type)
    }
    
    /// Power-up getters for game systems that need them
    pub fn get_head_powerup(&self) -> Option<PowerUpType> {
        self.slots.get(0).map(|slot| slot.powerup)
    }
    
    pub fn get_torso_powerup(&self) -> Option<PowerUpType> {
        self.slots.get(1).map(|slot| slot.powerup)
    }
    
    pub fn get_legs_powerup(&self) -> Option<PowerUpType> {
        self.slots.get(2).map(|slot| slot.powerup)
    }
}
