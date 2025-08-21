pub mod damage;
pub mod effects;
pub mod projectiles;

use bevy::prelude::*;
use crate::game::player::Player;
use crate::game::enemy::Enemy;
use crate::game::movement::Collider;
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


pub fn handle_combat(
    // keep if you emit events; prefix with `_` if unused to silence warnings
    mut _events: EventWriter<CombatEvent>,
    mut q: ParamSet<(
        // Player query (disjoint from enemies)
        Query<(Entity, &Transform, &mut Health, &CombatStats, &Collider),
              (With<Player>, Without<Enemy>)>,
        // Enemy query (disjoint from player)
        Query<(Entity, &Transform, &mut Health, &CombatStats, &Collider),
              (With<Enemy>, Without<Player>)>,
    )>,
    // keep or remove if unused
    _time: Res<Time>,
) {
    // ---- Phase 1: snapshot only what we need from the player, then drop the borrow ----
    let (player_entity, player_pos) = {
        // Create a named binding so the borrow of p0() ends with this block
        let mut p0 = q.p0();
        let Ok((pe, ptf, _ph, _ps, _pc)) = p0.single_mut() else { return };
        (pe, ptf.translation) // copy out the Vec3
    }; // p0 borrow ends here

    // If you deal damage to the player inside the enemy loop, accumulate it here
    let mut damage_to_player: i32 = 0;

    // Example naive range check; replace with your real collision math if you have it
    const ATTACK_RANGE: f32 = 24.0;

    // ---- Phase 2: iterate enemies (separate borrow) ----
    for (_enemy_entity, enemy_tf, _enemy_health, _enemy_stats, _enemy_collider) in q.p1().iter_mut() {
        let distance = player_pos.distance(enemy_tf.translation);

        if distance <= ATTACK_RANGE {
            // Example: accumulate some damage; replace with your damage calc
            damage_to_player += 1;

            // If you emit events, you could do:
            // _events.send(CombatEvent {
            //     attacker: _enemy_entity,
            //     target: player_entity,
            //     damage: 1,
            //     damage_type: DamageType::Physical,
            // });
        }

        // If you want to damage enemies here, change `_enemy_health` to `mut enemy_health`
        // and mutate it. Keeping the underscore avoids an "unused variable" warning.
    }

    // ---- Phase 3: apply accumulated damage to the player (new, short borrow of p0) ----
    if damage_to_player != 0 {
        let mut p0 = q.p0();
        if let Ok((_pe, _ptf, mut player_health, _ps, _pc)) = p0.single_mut() {
            // Apply the damage using your Health API/fields, e.g.:
            // player_health.current = (player_health.current - damage_to_player).max(0);
            //
            // or, if you have a helper:
            // player_health.apply_damage(damage_to_player);
        }
    }

    // keep `player_entity` "used" in case you remove event sending
    let _ = player_entity;
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
