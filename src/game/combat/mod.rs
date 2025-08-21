pub mod damage;
pub mod effects;
pub mod projectiles;

use bevy::prelude::*;
use crate::core::events::{CombatEvent, DamageType};

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
            regeneration: 0.0,
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

fn handle_combat(
    mut player_q: Query<(Entity, &Transform, &mut Health, &CombatStats, &crate::game::movement::Collider), With<crate::game::player::Player>>,
    mut enemy_q: Query<(Entity, &Transform, &mut Health, &CombatStats, &crate::game::movement::Collider), With<crate::game::enemy::Enemy>>,
    mut combat_events: EventWriter<CombatEvent>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player_tf, mut player_health, player_stats, player_collider)) = player_q.get_single_mut() {
        for (enemy_entity, enemy_tf, mut enemy_health, enemy_stats, enemy_collider) in enemy_q.iter_mut() {
            let distance = player_tf.translation.distance(enemy_tf.translation);
            let collision_distance = (player_collider.size.x + enemy_collider.size.x) / 2.0;
            
            if distance < collision_distance {
                // Enemy damages player
                combat_events.write(CombatEvent {
                    attacker: enemy_entity,
                    target: player_entity,
                    damage: enemy_stats.damage,
                    damage_type: DamageType::Physical,
                    position: player_tf.translation,
                });
                
                // Player damages enemy (simplified melee)
                combat_events.write(CombatEvent {
                    attacker: player_entity,
                    target: enemy_entity,
                    damage: player_stats.damage,
                    damage_type: DamageType::Physical,
                    position: enemy_tf.translation,
                });
            }
        }
    }
}

fn cleanup_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
    mut state: ResMut<crate::core::state::GameStats>,
) {
    for (entity, health) in query.iter() {
        if health.is_dead() {
            state.enemies_killed += 1;
            state.score += 10;
            commands.entity(entity).despawn();
        }
    }
}
