use bevy::prelude::*;
use super::*;

pub struct AbilityVisualsPlugin;

impl Plugin for AbilityVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_visual_effects,
            update_particles,
            update_trails,
            cleanup_expired_visuals,
        ));
    }
}

#[derive(Component)]
pub struct ParticleSystem {
    pub particles: Vec<ParticleData>,
    pub config: ParticleConfig,
    pub lifetime: Timer,
}

#[derive(Clone)]
pub struct ParticleData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub size: f32,
}

#[derive(Component)]
pub struct TrailEffect {
    pub positions: Vec<Vec3>,
    pub max_length: usize,
    pub fade_time: f32,
}

#[derive(Component)]
pub struct PulseEffect {
    pub base_scale: f32,
    pub pulse_scale: f32,
    pub pulse_speed: f32,
}

#[derive(Component)]
pub struct AuraEffect {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub rotation_speed: f32,
}

fn spawn_visual_effects(
    mut commands: Commands,
    mut events: EventReader<TriggerAbilityEvent>,
    registry: Res<AbilityRegistry>,
) {
    for event in events.read() {
        let Some(definition) = registry.abilities.get(&event.ability_id) else { continue };
        
        match &definition.visual_effect {
            VisualEffectType::Particles(config) => {
                spawn_particle_system(&mut commands, event.position, config.clone());
            }
            VisualEffectType::Trail => {
                // Trail effects are handled by the projectile/player movement
            }
            VisualEffectType::Pulse => {
                spawn_pulse_effect(&mut commands, event.position);
            }
            VisualEffectType::Aura => {
                spawn_aura_effect(&mut commands, event.caster);
            }
            _ => {}
        }
    }
}

fn spawn_particle_system(commands: &mut Commands, position: Vec3, config: ParticleConfig) {
    let mut particles = Vec::new();
    let mut rng = rand::thread_rng();
    
    for _ in 0..config.count {
        use rand::Rng;
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);
        
        particles.push(ParticleData {
            position: position.truncate(),
            velocity: Vec2::new(angle.cos(), angle.sin()) * speed,
            lifetime: config.lifetime,
            size: rng.gen_range(2.0..6.0),
        });
    }
    
    commands.spawn((
        ParticleSystem {
            particles,
            config: config.clone(),
            lifetime: Timer::from_seconds(config.lifetime * 2.0, TimerMode::Once),
        },
        Transform::from_translation(position),
        Visibility::Visible,
    ));
}

fn spawn_pulse_effect(commands: &mut Commands, position: Vec3) {
    commands.spawn((
        PulseEffect {
            base_scale: 1.0,
            pulse_scale: 2.0,
            pulse_speed: 2.0,
        },
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.5),
            custom_size: Some(Vec2::splat(50.0)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 2.0)),
    ));
}

fn spawn_aura_effect(commands: &mut Commands, owner: Entity) {
    // This would attach to the owner entity
    commands.spawn((
        AuraEffect {
            inner_radius: 50.0,
            outer_radius: 100.0,
            rotation_speed: 1.0,
        },
        Sprite {
            color: Color::srgba(0.5, 0.5, 1.0, 0.3),
            custom_size: Some(Vec2::splat(200.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));
}

fn update_particles(
    mut particle_q: Query<(&mut ParticleSystem, &Transform)>,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    for (mut system, transform) in particle_q.iter_mut() {
        system.lifetime.tick(time.delta());
        
        for particle in system.particles.iter_mut() {
            particle.position += particle.velocity * time.delta_secs();
            particle.lifetime -= time.delta_secs();
            particle.velocity *= 0.98; // Drag
            
            if particle.lifetime > 0.0 {
                let alpha = particle.lifetime / system.config.lifetime;
                let mut color = system.config.color;
                color.set_alpha(color.alpha() * alpha);
                
                gizmos.circle_2d(
                    particle.position,
                    particle.size,
                    color,
                );
            }
        }
        
        // Remove dead particles
        system.particles.retain(|p| p.lifetime > 0.0);
    }
}

fn update_trails(
    _trail_q: Query<&TrailEffect>,
    _time: Res<Time>,
) {
    // Trail update logic
}

fn cleanup_expired_visuals(
    mut commands: Commands,
    particle_q: Query<(Entity, &ParticleSystem)>,
) {
    for (entity, system) in particle_q.iter() {
        if system.lifetime.finished() && system.particles.is_empty() {
            commands.entity(entity).despawn();
        }
    }
}
