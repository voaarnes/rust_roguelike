use bevy::prelude::*;
use crate::game::player::{Player, PlayerStats};
use crate::game::movement::Collider;
use crate::entities::powerup::PowerUpSlots;
use crate::systems::shop::PlayerCurrency;
use crate::systems::achievements::AchievementUnlockedEvent;
use crate::systems::quests::QuestCompleteEvent;
use crate::core::state::GameStats;

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
    mut currency_q: Query<&mut PlayerCurrency>,
    mut game_stats: ResMut<GameStats>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
    mut quest_events: EventWriter<QuestCompleteEvent>,
    player_entity_q: Query<Entity, With<Player>>,
) {
    let Ok((player_tf, mut player_stats)) = player_q.single_mut() else { return };
    let Ok(player_entity) = player_entity_q.get_single() else { return };
    
    for (collectible_entity, collectible_tf, collectible, _collider) in collectible_q.iter() {
        let distance = player_tf.translation.distance(collectible_tf.translation);
        
        // Check if close enough to pick up (within player + collectible radius)
        if distance < 40.0 {
            match collectible.collectible_type {
                CollectibleType::Coin => {
                    player_stats.coins_collected += collectible.value as u32;
                    game_stats.coins_collected += collectible.value as u32;
                    // Update shop currency as well
                    if let Ok(mut currency) = currency_q.single_mut() {
                        currency.coins += collectible.value as u32;
                    }
                    
                    // Trigger collection achievements
                    if game_stats.coins_collected >= 100 {
                        achievement_events.write(AchievementUnlockedEvent {
                            achievement_id: "coin_collector_bronze".to_string(),
                            player: player_entity,
                        });
                    }
                    if game_stats.coins_collected >= 1000 {
                        achievement_events.write(AchievementUnlockedEvent {
                            achievement_id: "coin_collector_silver".to_string(),
                            player: player_entity,
                        });
                    }
                    
                    // Trigger quest progress
                    quest_events.write(QuestCompleteEvent {
                        quest_id: "daily_collector".to_string(),
                        player: player_entity,
                    });
                }
                CollectibleType::Fruit(fruit_type) => {
                    if let Ok(mut powerup_slots) = powerup_q.single_mut() {
                        // Just add the fruit by its type - the ability system will handle the rest
                        if let Some(dropped) = powerup_slots.add_fruit_for_abilities(fruit_type) {
                            println!("Picked up fruit type {}, dropped: fruit type {}", fruit_type, dropped);
                        } else {
                            println!("Picked up fruit type {}", fruit_type);
                        }
                    }
                    
                    // Trigger fruit collection achievements
                    achievement_events.write(AchievementUnlockedEvent {
                        achievement_id: "fruit_collector".to_string(),
                        player: player_entity,
                    });
                }
                CollectibleType::Gem => {
                    // Update shop currency
                    if let Ok(mut currency) = currency_q.single_mut() {
                        currency.gems += collectible.value as u32;
                    }
                    
                    // Trigger gem collection achievements
                    achievement_events.write(AchievementUnlockedEvent {
                        achievement_id: "gem_collector".to_string(),
                        player: player_entity,
                    });
                }
                CollectibleType::HealthPotion => {
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
