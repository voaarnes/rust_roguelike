use bevy::prelude::*;
use crate::core::events::{CombatEvent, DamageType};
use crate::game::combat::{Health, CombatStats, DamageImmunity};

#[derive(Component)]
pub struct DamageNumber {
    pub value: i32,
    pub color: Color,
    pub velocity: Vec2,
    pub lifetime: Timer,
}

pub fn process_damage_events(
    mut combat_events: EventReader<CombatEvent>,
    mut health_q: Query<(&mut Health, &CombatStats, Option<&mut DamageImmunity>)>,
    mut commands: Commands,
) {
    for event in combat_events.read() {
        if let Ok((mut health, stats, immunity)) = health_q.get_mut(event.target) {
            // Check immunity
            if immunity.is_some() {
                continue;
            }
            
            // Calculate damage
            let mut final_damage = event.damage;
            
            // Apply armor
            final_damage = (final_damage - stats.armor).max(1);
            
            // Apply damage type modifiers
            final_damage = match event.damage_type {
                DamageType::True => event.damage, // Ignores armor
                DamageType::Magic => (final_damage as f32 * 1.2) as i32,
                _ => final_damage,
            };
            
            // Apply damage
            health.take_damage(final_damage);
            
            // Spawn damage number
            spawn_damage_number(&mut commands, event.position, final_damage, event.damage_type);
            
            // Add temporary immunity
            commands.entity(event.target).insert(DamageImmunity {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            });
        }
    }
}

fn spawn_damage_number(
    commands: &mut Commands,
    position: Vec3,
    damage: i32,
    damage_type: DamageType,
) {
    let color = match damage_type {
        DamageType::Physical => Color::WHITE,
        DamageType::Magic => Color::linear_rgb(0.5, 0.0, 1.0),
        DamageType::Fire => Color::linear_rgb(1.0, 0.5, 0.0),
        DamageType::Ice => Color::linear_rgb(0.0, 0.5, 1.0),
        DamageType::Poison => Color::linear_rgb(0.0, 1.0, 0.0),
        DamageType::True => Color::linear_rgb(1.0, 1.0, 0.0),
    };
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                damage.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color,
                    ..default()
                },
            ),
            transform: Transform::from_translation(position + Vec3::new(0.0, 20.0, 100.0)),
            ..default()
        },
        DamageNumber {
            value: damage,
            color,
            velocity: Vec2::new(
                (rand::random::<f32>() - 0.5) * 50.0,
                100.0,
            ),
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
        },
    ));
}

pub fn show_damage_numbers(
    mut query: Query<(Entity, &mut Transform, &mut Text, &mut DamageNumber)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut text, mut damage_num) in query.iter_mut() {
        damage_num.lifetime.tick(time.delta());
        
        // Move upward
        transform.translation += damage_num.velocity.extend(0.0) * time.delta_secs();
        damage_num.velocity.y -= 200.0 * time.delta_secs(); // Gravity
        
        // Fade out
        let alpha = damage_num.lifetime.fraction_remaining();
        if let Some(section) = text.sections.first_mut() {
            section.style.color.set_alpha(alpha);
        }
        
        // Remove when done
        if damage_num.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
