// src/entities/powerup.rs

use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone)]
pub struct PowerUpSlots {
    pub slots: VecDeque<FruitSlot>,  // Changed from PowerUpType to FruitSlot
    pub max_slots: usize,
}

// New struct to store both fruit type and power-up type
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FruitSlot {
    pub fruit_type: u8,           // 0-7 for different fruits
    pub powerup: PowerUpType,     // The actual power-up effect
}

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
    
    // Changed to accept fruit_type as well
    pub fn add_fruit(&mut self, fruit_type: u8, powerup: PowerUpType) -> Option<FruitSlot> {
        let fruit_slot = FruitSlot { fruit_type, powerup };
        
        // Debug print the state before adding
        println!("Before adding fruit {} ({:?}):", fruit_type, powerup);
        println!("  Current slots: {:?}", self.slots);
        
        let dropped = if self.slots.len() >= self.max_slots {
            self.slots.pop_back() // Remove oldest (back of deque)
        } else {
            None
        };
        
        self.slots.push_front(fruit_slot); // Add newest to front
        
        // Debug print the state after adding
        println!("After adding:");
        println!("  New slots: {:?}", self.slots);
        println!("  Dropped: {:?}", dropped);
        
        dropped
    }
    
    // Keep the old method for backward compatibility but mark as deprecated
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
    
    // Get the newest fruit (head) - index 0 in deque
    pub fn get_head_fruit(&self) -> Option<u8> {
        self.slots.get(0).map(|slot| slot.fruit_type)
    }
    
    // Get the middle fruit (torso) - index 1 in deque
    pub fn get_torso_fruit(&self) -> Option<u8> {
        self.slots.get(1).map(|slot| slot.fruit_type)
    }
    
    // Get the oldest fruit (legs) - index 2 in deque
    pub fn get_legs_fruit(&self) -> Option<u8> {
        self.slots.get(2).map(|slot| slot.fruit_type)
    }
    
    // Keep the old power-up getters for other systems that need them
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
