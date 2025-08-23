use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone)]
pub struct PowerUpSlots {
    pub slots: VecDeque<PowerUpType>,
    pub max_slots: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerUpType {
    SpeedBoost,      // Strawberry/Pear (indices 0,1)
    DamageBoost,     // Mango/Apple (indices 2,3)
    HealthBoost,     // Orange/Grape (indices 4,5)
    ShieldBoost,     // Banana/Cherry (indices 6,7)
}

impl PowerUpSlots {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: VecDeque::with_capacity(max_slots),
            max_slots,
        }
    }
    
    pub fn add_powerup(&mut self, powerup: PowerUpType) -> Option<PowerUpType> {
        let dropped = if self.slots.len() >= self.max_slots {
            self.slots.pop_back() // Remove oldest (FIFO)
        } else {
            None
        };
        
        self.slots.push_front(powerup); // Add newest to front
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
    
    // Get the newest fruit (head)
    pub fn get_head_fruit(&self) -> Option<PowerUpType> {
        self.slots.front().copied()
    }
    
    // Get the middle fruit (torso/chest)
    pub fn get_torso_fruit(&self) -> Option<PowerUpType> {
        if self.slots.len() >= 2 {
            self.slots.get(1).copied()
        } else {
            None
        }
    }
    
    // Get the oldest fruit (legs)
    pub fn get_legs_fruit(&self) -> Option<PowerUpType> {
        if self.slots.len() >= 3 {
            self.slots.get(2).copied()
        } else if self.slots.len() >= 2 {
            // If we only have 2 fruits, legs get the oldest one
            self.slots.get(1).copied()
        } else {
            None
        }
    }
}
