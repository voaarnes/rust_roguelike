use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};
use crate::game::player::Player;

pub struct AbilityTestPlugin;

impl Plugin for AbilityTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, test_ability_setup);
    }
}

fn test_ability_setup(
    mut player_q: Query<&mut PowerUpSlots, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // Press number keys to test different fruit combinations
    if keys.just_pressed(KeyCode::Digit1) {
        if let Ok(mut slots) = player_q.single_mut() {
            // Test strawberry abilities
            slots.add_fruit(0, PowerUpType::SpeedBoost);
            println!("Added Strawberry - check for Rapid Fire ability!");
        }
    }
    
    if keys.just_pressed(KeyCode::Digit2) {
        if let Ok(mut slots) = player_q.single_mut() {
            // Test mango abilities
            slots.add_fruit(2, PowerUpType::DamageBoost);
            println!("Added Mango - check for Explosive abilities!");
        }
    }
    
    if keys.just_pressed(KeyCode::Digit3) {
        if let Ok(mut slots) = player_q.single_mut() {
            // Test coconut abilities
            slots.add_fruit(6, PowerUpType::ShieldBoost);
            println!("Added Coconut - check for defensive abilities!");
        }
    }
    
    if keys.just_pressed(KeyCode::Digit4) {
        if let Ok(mut slots) = player_q.single_mut() {
            // Test apple abilities
            slots.add_fruit(4, PowerUpType::HealthBoost);
            println!("Added Apple - check for gravity/life steal abilities!");
        }
    }
}
