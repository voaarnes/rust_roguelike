pub mod damage;
pub mod effects;
pub mod projectiles;

use bevy::prelude::*;
use crate::game::player::Player;
use crate::game::enemy::Enemy;
use crate::game::movement::Collider;
use crate::systems::loot::{DropLootEvent, LootSource};
use crate::systems::combo::ComboEvent;
use crate::systems::achievements::AchievementUnlockedEvent;
use crate::systems::quests::QuestCompleteEvent;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_combat,
                damage::process_damage_events,
                damage::show_damage_numbers,
                effects::update_status_effects,
                projectiles::update_projectiles,
                cleanup_dead_entities,
                health_regeneration,
            ));
    }
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub regeneration: f32,
    pub regen_timer: Timer,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max,
            regeneration: 1.0, // 1 HP per second
            regen_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
    
    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
    
    pub fn take_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }
    
    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }
}

#[derive(Component)]
pub struct CombatStats {
    pub damage: i32,
    pub armor: i32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
}

#[derive(Component)]
pub struct DamageImmunity {
    pub timer: Timer,
}

#[derive(Component)]
pub struct LastDamageTime {
    pub timer: Timer,
}

impl Default for LastDamageTime {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

pub fn handle_combat(
    mut player_q: Query<(Entity, &Transform, &mut Health, &CombatStats, &Collider, Option<&mut LastDamageTime>), (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(&Transform, &mut Health, &CombatStats, &Collider), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut combo_events: EventWriter<ComboEvent>,
    time: Res<Time>,
) {
    let Ok((player_entity, player_tf, mut player_health, player_stats, player_collider, player_damage_time)) = player_q.single_mut() else { return };
    
    // Update player damage immunity timer
    let mut can_take_damage = true;
    if let Some(mut damage_time) = player_damage_time {
        damage_time.timer.tick(time.delta());
        can_take_damage = damage_time.timer.finished();
    }
    
    for (enemy_tf, mut enemy_health, enemy_stats, enemy_collider) in enemy_q.iter_mut() {
        let distance = player_tf.translation.distance(enemy_tf.translation);
        let collision_distance = (player_collider.size.x + enemy_collider.size.x) / 2.0;
        
        // Check collision for damage
        if distance <= collision_distance {
            // Enemy damages player
            if can_take_damage {
                let damage = (enemy_stats.damage - player_stats.armor).max(1);
                player_health.take_damage(damage);
                
                // Reset combo when player takes damage
                combo_events.write(ComboEvent::Reset);
                
                // Add damage immunity period
                commands.entity(player_entity).insert(LastDamageTime::default());
            }
            
            // Player damages enemy (continuous damage when touching)
            enemy_health.take_damage(1);
            
            // Send combo event for hit
            combo_events.write(ComboEvent::Hit);
        }
    }
}

fn health_regeneration(
    mut query: Query<&mut Health>,
    time: Res<Time>,
) {
    for mut health in query.iter_mut() {
        health.regen_timer.tick(time.delta());
        if health.regen_timer.just_finished() && health.current < health.max {
            let regen_amount = health.regeneration as i32;
            health.heal(regen_amount);
        }
    }
}

fn cleanup_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform, Option<&Enemy>)>,
    mut state: ResMut<crate::core::state::GameStats>,
    mut loot_events: EventWriter<DropLootEvent>,
    mut combo_events: EventWriter<ComboEvent>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
    mut quest_events: EventWriter<QuestCompleteEvent>,
    player_q: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_q.get_single() else { return };
    
    for (entity, health, transform, enemy) in query.iter() {
        if health.is_dead() {
            state.enemies_killed += 1;
            state.score += 10;
            
            // Send combo event for kill
            combo_events.write(ComboEvent::Kill);
            
            // Drop loot if this was an enemy
            if let Some(_enemy) = enemy {
                loot_events.write(DropLootEvent {
                    position: transform.translation,
                    source: LootSource::Enemy("BasicEnemy".to_string()),
                    luck_bonus: 0.0, // TODO: Get from player stats
                });
                
                // Trigger achievement progress for enemy kills
                achievement_events.write(AchievementUnlockedEvent {
                    achievement_id: "first_kill".to_string(),
                    player: player_entity,
                });
                
                // Trigger quest progress for enemy kills
                quest_events.write(QuestCompleteEvent {
                    quest_id: "daily_slayer".to_string(),
                    player: player_entity,
                });
                
                // Check for milestone achievements
                if state.enemies_killed == 10 {
                    achievement_events.write(AchievementUnlockedEvent {
                        achievement_id: "slayer_bronze".to_string(),
                        player: player_entity,
                    });
                } else if state.enemies_killed == 100 {
                    achievement_events.write(AchievementUnlockedEvent {
                        achievement_id: "slayer_silver".to_string(),
                        player: player_entity,
                    });
                } else if state.enemies_killed == 1000 {
                    achievement_events.write(AchievementUnlockedEvent {
                        achievement_id: "slayer_gold".to_string(),
                        player: player_entity,
                    });
                }
            }
            
            commands.entity(entity).despawn();
        }
    }
}
