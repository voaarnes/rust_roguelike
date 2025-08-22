use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone)]
pub struct PowerUpSlots {
    pub slots: VecDeque<PowerUpType>,
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
    
    pub fn get_head_fruit(&self) -> Option<PowerUpType> {
        self.slots.front().copied()
    }
    
    pub fn get_legs_fruit(&self) -> Option<PowerUpType> {
        if self.slots.len() >= 2 {
            self.slots.get(self.slots.len() - 1).copied()
        } else {
            None
        }
    }
}
