
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone)]
pub struct PowerUpSlots {
    pub slots: VecDeque<PowerUpType>,
    pub max_slots: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerUpType {
    SpeedBoost,      // Strawberry/Pear
    DamageBoost,     // Mango/Apple  
    HealthBoost,     // Orange/Grape
    ShieldBoost,     // Banana/Cherry
}

impl PowerUpSlots {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: VecDeque::with_capacity(max_slots),
            max_slots,
        }
    }
    
    pub fn add_powerup(&mut self, powerup: PowerUpType) -> Option<PowerUpType> {
        // Debug print the state before adding
        println!("Before adding {:?}:", powerup);
        println!("  Current slots: {:?}", self.slots);
        
        let dropped = if self.slots.len() >= self.max_slots {
            self.slots.pop_back() // Remove oldest (back of deque)
        } else {
            None
        };
        
        self.slots.push_front(powerup); // Add newest to front
        
        // Debug print the state after adding
        println!("After adding:");
        println!("  New slots: {:?}", self.slots);
        println!("  Dropped: {:?}", dropped);
        
        dropped
    }
    
    pub fn get_slots_as_vec(&self) -> Vec<Option<PowerUpType>> {
        let mut vec = Vec::with_capacity(self.max_slots);
        for i in 0..self.max_slots {
            if i < self.slots.len() {
                vec.push(Some(self.slots[i]));
            } else {
                vec.push(None);
            }
        }
        vec
    }
    
    // Get the newest fruit (head) - index 0 in deque
    pub fn get_head_fruit(&self) -> Option<PowerUpType> {
        self.slots.get(0).copied()
    }
    
    // Get the middle fruit (torso) - index 1 in deque
    pub fn get_torso_fruit(&self) -> Option<PowerUpType> {
        self.slots.get(1).copied()
    }
    
    // Get the oldest fruit (legs) - index 2 in deque
    pub fn get_legs_fruit(&self) -> Option<PowerUpType> {
        self.slots.get(2).copied()
    }
}
