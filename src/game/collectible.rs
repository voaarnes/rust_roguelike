use bevy::prelude::*;
use crate::game::player::{Player, PlayerStats};
use crate::game::movement::Collider;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_collectible_pickup,
            animate_collectibles,
        ));
    }
}

#[derive(Component)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub value: i32,
}

#[derive(Clone, Copy)]
pub enum CollectibleType {
    Coin,
    Gem,
    HealthPotion,
    ManaPotion,
    Fruit(u8), // 0-7 for different fruit types
}

fn handle_collectible_pickup(
    mut commands: Commands,
    mut player_q: Query<(&Transform, &mut PlayerStats), With<Player>>,
    collectible_q: Query<(Entity, &Transform, &Collectible, &Collider)>,
    mut powerup_q: Query<&mut PowerUpSlots, With<Player>>,
) {
    let Ok((player_tf, mut player_stats)) = player_q.single_mut() else { return };
    
    for (collectible_entity, collectible_tf, collectible, _collider) in collectible_q.iter() {
        let distance = player_tf.translation.distance(collectible_tf.translation);
        
        // Check if close enough to pick up (within player + collectible radius)
        if distance < 40.0 {
            match collectible.collectible_type {
                CollectibleType::Coin => {
                    player_stats.coins_collected += collectible.value as u32;
                    println!("Picked up {} coins! Total: {}", collectible.value, player_stats.coins_collected);
                }


                CollectibleType::Fruit(fruit_type) => {
                    if let Ok(mut powerup_slots) = powerup_q.single_mut() {
                        let powerup = match fruit_type {
                            0 | 1 => PowerUpType::SpeedBoost,      // Strawberry, Pear
                            2 | 3 => PowerUpType::DamageBoost,     // Mango, Apple
                            4 | 5 => PowerUpType::HealthBoost,     // Orange, Grape
                            6 | 7 => PowerUpType::ShieldBoost,     // Banana, Cherry
                            _ => PowerUpType::SpeedBoost,
                        };
        
                        // Now correctly handles Option<PowerUpType> return
                        if let Some(dropped) = powerup_slots.add_powerup(powerup) {
                            println!("Gained power-up: {:?}, dropped: {:?}", powerup, dropped);
                        } else {
                            println!("Gained power-up: {:?}", powerup);
                        }
                    }
                }
                CollectibleType::HealthPotion => {
                    // Handle health potion
                    println!("Picked up health potion!");
                }
                _ => {}
            }
            
            // Remove the collectible
            commands.entity(collectible_entity).despawn();
        }
    }
}

fn animate_collectibles(
    mut query: Query<&mut Transform, With<Collectible>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        // Add a subtle floating animation
        transform.translation.y += (time.elapsed_secs() * 3.0 + transform.translation.x * 0.01).sin() * 0.3;
        transform.rotation = Quat::from_rotation_z((time.elapsed_secs() * 2.0).sin() * 0.1);
    }
}

// Update fruit pickup to use FIFO queue
fn handle_fruit_pickup(
    mut commands: Commands,
    mut player_q: Query<(&Transform, &mut crate::game::player::PlayerStats), With<crate::game::player::Player>>,
    collectible_q: Query<(Entity, &Transform, &Collectible, &crate::game::movement::Collider)>,
    mut powerup_q: Query<&mut crate::entities::powerup::PowerUpSlots, With<crate::game::player::Player>>,
) {
    let Ok((player_tf, mut player_stats)) = player_q.single_mut() else { return };
    
    for (collectible_entity, collectible_tf, collectible, _collider) in collectible_q.iter() {
        let distance = player_tf.translation.distance(collectible_tf.translation);
        
        // Check if close enough to pick up (within player + collectible radius)
        if distance < 40.0 {
            match collectible.collectible_type {
                CollectibleType::Coin => {
                    player_stats.coins_collected += collectible.value as u32;
                    println!("Picked up {} coins! Total: {}", collectible.value, player_stats.coins_collected);
                }
                CollectibleType::Fruit(fruit_type) => {
                    if let Ok(mut powerup_slots) = powerup_q.single_mut() {
                        let powerup = match fruit_type {
                            0 | 1 => crate::entities::powerup::PowerUpType::SpeedBoost,      // Strawberry, Pear
                            2 | 3 => crate::entities::powerup::PowerUpType::DamageBoost,     // Mango, Apple
                            4 | 5 => crate::entities::powerup::PowerUpType::HealthBoost,     // Orange, Grape
                            6 | 7 => crate::entities::powerup::PowerUpType::ShieldBoost,     // Banana, Cherry
                            _ => crate::entities::powerup::PowerUpType::SpeedBoost,
                        };
                        
                        // Use FIFO queue to add powerup
                        if let Some(dropped) = powerup_slots.add_powerup(powerup) {
                            println!("Gained power-up: {:?}, dropped: {:?}", powerup, dropped);
                        } else {
                            println!("Gained power-up: {:?}", powerup);
                        }
                    }
                }
                _ => {}
            }
            
            // Remove the collectible
            commands.entity(collectible_entity).despawn();
        }
    }
}
