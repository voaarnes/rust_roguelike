use bevy::prelude::*;
use super::*;
use crate::core::camera::CameraShake;

pub struct AbilityVisualsPlugin;

impl Plugin for AbilityVisualsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FruitVisualAssets>()
            .add_systems(Startup, load_fruit_visual_assets)
            .add_systems(
                Update,
                (
                    spawn_visual_effects,
                    update_particles,
                    update_trails,
                    update_pulse_effects,
                ),
            )
            .add_systems(
                Update,
                (
                    update_aura_effects,
                    animate_projectile_sprites,
                    animate_area_effect_sprites,
                    update_chromatic_aberration,
                ),
            )
            .add_systems(Update, cleanup_expired_visuals);
    }
}

#[derive(Resource, Default)]
pub struct FruitVisualAssets {
    pub projectile_atlas: Handle<TextureAtlasLayout>,
    pub projectile_texture: Handle<Image>,
    pub area_effect_atlas: Handle<TextureAtlasLayout>,
    pub area_effect_texture: Handle<Image>,
    pub particle_atlas: Handle<TextureAtlasLayout>,
    pub particle_texture: Handle<Image>,
    pub trail_atlas: Handle<TextureAtlasLayout>,
    pub trail_texture: Handle<Image>,
    pub buff_atlas: Handle<TextureAtlasLayout>,
    pub buff_texture: Handle<Image>,
    pub summon_atlas: Handle<TextureAtlasLayout>,
    pub summon_texture: Handle<Image>,
}

fn load_fruit_visual_assets(
    mut assets: ResMut<FruitVisualAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load projectile sprites
    assets.projectile_texture = asset_server.load("sprites/fruit_projectiles.png");
    assets.projectile_atlas = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8, 4,
        None, None,
    ));

    // Load area effect sprites
    assets.area_effect_texture = asset_server.load("sprites/fruit_area_effects.png");
    assets.area_effect_atlas = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        6, 4,
        None, None,
    ));

    // Load particle sprites
    assets.particle_texture = asset_server.load("sprites/fruit_particles.png");
    assets.particle_atlas = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        8, 8,
        None, None,
    ));

    // Load trail sprites
    assets.trail_texture = asset_server.load("sprites/fruit_trails.png");
    assets.trail_atlas = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8, 1,
        None, None,
    ));

    // Load buff sprites
    assets.buff_texture = asset_server.load("sprites/fruit_buffs.png");
    assets.buff_atlas = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        4, 4,
        None, None,
    ));

    // Load summon sprites
    assets.summon_texture = asset_server.load("sprites/fruit_summons.png");
    assets.summon_atlas = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        8, 4,
        None, None,
    ));
}

#[derive(Component)]
pub struct AnimatedSprite {
    pub first_frame: usize,
    pub frame_count: usize,
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub looping: bool,
}

#[derive(Component)]
pub struct ProjectileVisualEffect {
    pub fruit_type: ProjectileVisual,
    pub glow_intensity: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct ParticleSystem {
    pub particles: Vec<ParticleData>,
    pub config: ParticleConfig,
    pub lifetime: Timer,
    pub fruit_type: u8,
}

#[derive(Component, Clone)]
pub struct ParticleData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub size: f32,
    pub rotation: f32,
    pub particle_type: usize, // Index into particle atlas
}

#[derive(Component)]
pub struct TrailEffect {
    pub positions: Vec<Vec3>,
    pub max_length: usize,
    pub fade_time: f32,
    pub fruit_type: u8,
    pub segment_entities: Vec<Entity>,
}

#[derive(Component)]
pub struct TrailSegment {
    pub lifetime: Timer,
    pub initial_alpha: f32,
}

#[derive(Component)]
pub struct PulseEffect {
    pub base_scale: f32,
    pub pulse_scale: f32,
    pub pulse_speed: f32,
    pub lifetime: Timer,
    pub shockwave: bool,
    pub fruit_type: u8,
}

#[derive(Component)]
pub struct AuraEffect {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub rotation_speed: f32,
    pub lifetime: Timer,
    pub fruit_type: u8,
    pub particle_spawn_timer: Timer,
}

#[derive(Component)]
pub struct ChromaticAberration {
    pub intensity: f32,
    pub duration: Timer,
}

fn spawn_visual_effects(
    mut commands: Commands,
    mut events: EventReader<TriggerAbilityEvent>,
    registry: Res<AbilityRegistry>,
    assets: Res<FruitVisualAssets>,
    mut camera_q: Query<Entity, With<Camera2d>>,
) {
    for event in events.read() {
        let Some(definition) = registry.abilities.get(&event.ability_id) else { continue };
        
        match &definition.visual_effect {
            VisualEffectType::Particles(config) => {
                spawn_enhanced_particle_system(
                    &mut commands, 
                    event.position, 
                    config.clone(),
                    event.ability_id.fruit_type,
                    &assets,
                );
            }
            VisualEffectType::Trail => {
                spawn_trail_effect(&mut commands, event.caster, event.ability_id.fruit_type);
            }
            VisualEffectType::Pulse => {
                spawn_pulse_effect(
                    &mut commands, 
                    event.position,
                    event.ability_id.fruit_type,
                    &assets,
                );
            }
            VisualEffectType::Aura => {
                spawn_aura_effect(
                    &mut commands, 
                    event.caster,
                    event.ability_id.fruit_type,
                    &assets,
                );
            }
            _ => {}
        }

        // Add screen effects for powerful abilities
        match event.ability_id.fruit_type {
            2 => { // Mango explosion
                println!("Spawning Mango screen shake!");
                spawn_screen_shake(&mut commands, 0.5, 0.3, &mut camera_q);
            }
            6 => { // Coconut earthquake
                println!("Spawning Coconut earthquake shake!");
                spawn_screen_shake(&mut commands, 1.0, 0.5, &mut camera_q);
                spawn_chromatic_aberration(&mut commands, 0.3, 0.2);
            }
            _ => {}
        }
    }
}

fn spawn_enhanced_particle_system(
    commands: &mut Commands,
    position: Vec3,
    config: ParticleConfig,
    fruit_type: u8,
    assets: &FruitVisualAssets,
) {
    let mut particles = Vec::new();
    let mut rng = rand::thread_rng();
    
    // Different particle patterns based on fruit type
    let particle_pattern = match fruit_type {
        0 => ParticlePattern::Burst,     // Strawberry
        1 => ParticlePattern::Fountain,  // Pear
        2 => ParticlePattern::Explosion, // Mango
        3 => ParticlePattern::Spiral,    // Pineapple
        4 => ParticlePattern::Vortex,    // Apple
        5 => ParticlePattern::Cone,      // Carrot
        6 => ParticlePattern::Ring,      // Coconut
        _ => ParticlePattern::Burst,
    };

    for i in 0..config.count {
        use rand::Rng;
        let (velocity, particle_type) = generate_particle_velocity(
            &particle_pattern, 
            i, 
            config.count,
            &mut rng,
            fruit_type,
        );

        particles.push(ParticleData {
            position: position.truncate(),
            velocity,
            lifetime: config.lifetime * rng.gen_range(0.8..1.2),
            size: rng.gen_range(4.0..12.0),
            rotation: rng.gen_range(0.0..std::f32::consts::TAU),
            particle_type,
        });
    }

    commands.spawn((
        ParticleSystem {
            particles,
            config: config.clone(),
            lifetime: Timer::from_seconds(config.lifetime * 2.0, TimerMode::Once),
            fruit_type,
        },
        Transform::from_translation(position),
        Visibility::Visible,
    ));
}

#[derive(Clone, Copy)]
enum ParticlePattern {
    Burst,
    Fountain,
    Explosion,
    Spiral,
    Vortex,
    Cone,
    Ring,
}

fn generate_particle_velocity(
    pattern: &ParticlePattern,
    index: u32,
    total: u32,
    rng: &mut impl rand::Rng,
    fruit_type: u8,
) -> (Vec2, usize) {
    use rand::Rng;
    
    let particle_type = (fruit_type as usize * 8) + rng.gen_range(0..8);
    
    let velocity = match pattern {
        ParticlePattern::Burst => {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(100.0..300.0);
            Vec2::new(angle.cos(), angle.sin()) * speed
        }
        ParticlePattern::Fountain => {
            let angle = rng.gen_range(-0.5..0.5);
            let speed = rng.gen_range(200.0..400.0);
            Vec2::new(angle, 1.0).normalize() * speed
        }
        ParticlePattern::Explosion => {
            let angle = (index as f32 / total as f32) * std::f32::consts::TAU;
            let speed = rng.gen_range(300.0..500.0);
            Vec2::new(angle.cos(), angle.sin()) * speed
        }
        ParticlePattern::Spiral => {
            let angle = (index as f32 / total as f32) * std::f32::consts::TAU * 2.0;
            let radius = index as f32 / total as f32;
            let speed = 200.0 + radius * 100.0;
            Vec2::new(angle.cos(), angle.sin()) * speed
        }
        ParticlePattern::Vortex => {
            let angle = (index as f32 / total as f32) * std::f32::consts::TAU;
            let tangent = Vec2::new(-angle.sin(), angle.cos());
            let radial = Vec2::new(angle.cos(), angle.sin());
            (tangent * 200.0 + radial * 50.0) * rng.gen_range(0.8..1.2)
        }
        ParticlePattern::Cone => {
            let spread = 0.5;
            let angle = rng.gen_range(-spread..spread);
            let speed = rng.gen_range(250.0..450.0);
            Vec2::new(angle, -1.0).normalize() * speed
        }
        ParticlePattern::Ring => {
            let angle = (index as f32 / total as f32) * std::f32::consts::TAU;
            let speed = 250.0;
            Vec2::new(angle.cos(), angle.sin()) * speed
        }
    };
    
    (velocity, particle_type)
}

fn spawn_trail_effect(commands: &mut Commands, entity: Entity, fruit_type: u8) {
    commands.entity(entity).insert(TrailEffect {
        positions: Vec::new(),
        max_length: 20,
        fade_time: 0.5,
        fruit_type,
        segment_entities: Vec::new(),
    });
}

fn spawn_pulse_effect(
    commands: &mut Commands,
    position: Vec3,
    fruit_type: u8,
    assets: &FruitVisualAssets,
) {
    // Create expanding ring effect
    commands.spawn((
        PulseEffect {
            base_scale: 0.1,
            pulse_scale: 5.0,
            pulse_speed: 3.0,
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
            shockwave: true,
            fruit_type,
        },
        Sprite {
            image: assets.area_effect_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.area_effect_atlas.clone(),
                index: 0,
            }),
            color: get_fruit_color(fruit_type).with_alpha(0.6),
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 2.0)),
    ));
}

fn spawn_aura_effect(
    commands: &mut Commands,
    owner: Entity,
    fruit_type: u8,
    assets: &FruitVisualAssets,
) {
    let aura_entity = commands.spawn((
        AuraEffect {
            inner_radius: 50.0,
            outer_radius: 150.0,
            rotation_speed: 1.0,
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            fruit_type,
            particle_spawn_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        Sprite {
            image: assets.buff_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.buff_atlas.clone(),
                index: fruit_type as usize,
            }),
            color: get_fruit_color(fruit_type).with_alpha(0.3),
            custom_size: Some(Vec2::splat(200.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    )).id();
    
    // Attach to owner
    commands.entity(owner).add_child(aura_entity);
}

fn spawn_screen_shake(
    commands: &mut Commands, 
    intensity: f32, 
    duration: f32,
    camera_q: &mut Query<Entity, With<Camera2d>>,
) {
    if let Ok(camera_entity) = camera_q.single() {
        commands.entity(camera_entity).insert(CameraShake {
            intensity,
            duration: Timer::from_seconds(duration, TimerMode::Once),
        });
    }
}

fn spawn_chromatic_aberration(commands: &mut Commands, intensity: f32, duration: f32) {
    commands.spawn(ChromaticAberration {
        intensity,
        duration: Timer::from_seconds(duration, TimerMode::Once),
    });
}

fn get_fruit_color(fruit_type: u8) -> Color {
    match fruit_type {
        0 => Color::srgb(1.0, 0.2, 0.4),      // Strawberry
        1 => Color::srgb(0.7, 1.0, 0.3),      // Pear
        2 => Color::srgb(1.0, 0.7, 0.0),      // Mango
        3 => Color::srgb(1.0, 0.84, 0.0),     // Pineapple
        4 => Color::srgb(0.86, 0.08, 0.24),   // Apple
        5 => Color::srgb(1.0, 0.42, 0.21),    // Carrot
        6 => Color::srgb(0.55, 0.41, 0.31),   // Coconut
        _ => Color::WHITE,
    }
}

fn animate_projectile_sprites(
    mut query: Query<(&mut Sprite, &mut AnimatedSprite, &ProjectileVisualEffect)>,
    time: Res<Time>,
) {
    for (mut sprite, mut anim, visual) in query.iter_mut() {
        anim.frame_timer.tick(time.delta());
        
        if anim.frame_timer.just_finished() {
            anim.current_frame = if anim.looping {
                (anim.current_frame + 1) % anim.frame_count
            } else {
                (anim.current_frame + 1).min(anim.frame_count - 1)
            };
            
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = anim.first_frame + anim.current_frame;
            }
        }
    }
}

fn animate_area_effect_sprites(
    mut query: Query<(&mut Sprite, &mut AnimatedSprite), Without<ProjectileVisualEffect>>,
    time: Res<Time>,
) {
    for (mut sprite, mut anim) in query.iter_mut() {
        anim.frame_timer.tick(time.delta());
        
        if anim.frame_timer.just_finished() && !anim.frame_timer.paused() {
            if anim.current_frame < anim.frame_count - 1 {
                anim.current_frame += 1;
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = anim.first_frame + anim.current_frame;
                }
            } else if !anim.looping {
                anim.frame_timer.pause();
            } else {
                anim.current_frame = 0;
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = anim.first_frame;
                }
            }
        }
    }
}

fn update_particles(
    mut particle_q: Query<(&mut ParticleSystem, &Transform)>,
    mut gizmos: Gizmos,
    time: Res<Time>,
    assets: Res<FruitVisualAssets>,
) {
    for (mut system, transform) in particle_q.iter_mut() {
        system.lifetime.tick(time.delta());
        
        let gravity = Vec2::new(0.0, -50.0);
        let drag = 0.98;
        
        // Extract values before mutable borrow
        let config_lifetime = system.config.lifetime;
        let fruit_type = system.fruit_type;
        
        for particle in system.particles.iter_mut() {
            // Physics
            particle.velocity += gravity * time.delta_secs();
            particle.velocity *= drag;
            particle.position += particle.velocity * time.delta_secs();
            particle.lifetime -= time.delta_secs();
            particle.rotation += time.delta_secs() * 2.0;
            
            if particle.lifetime > 0.0 {
                let alpha = particle.lifetime / config_lifetime;
                let color = get_fruit_color(fruit_type).with_alpha(alpha);
                
                // Draw particle with rotation
                let world_pos = transform.translation.truncate() + particle.position;
                gizmos.circle_2d(world_pos, particle.size * alpha, color);
            }
        }
        
        // Remove dead particles
        system.particles.retain(|p| p.lifetime > 0.0);
    }
}

fn update_trails(
    mut trail_q: Query<(&mut TrailEffect, &GlobalTransform)>,
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<FruitVisualAssets>,
) {
    for (mut trail, transform) in trail_q.iter_mut() {
        let current_pos = transform.translation();
        
        // Add new position if moved enough
        if trail.positions.is_empty() || 
           trail.positions.last().map_or(true, |p| p.distance(current_pos) > 5.0) {
            trail.positions.push(current_pos);
            
            // Spawn trail segment sprite
            let segment = commands.spawn((
                TrailSegment {
                    lifetime: Timer::from_seconds(trail.fade_time, TimerMode::Once),
                    initial_alpha: 0.7,
                },
                Sprite {
                    image: assets.trail_texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: assets.trail_atlas.clone(),
                        index: trail.fruit_type as usize,
                    }),
                    color: get_fruit_color(trail.fruit_type).with_alpha(0.7),
                    custom_size: Some(Vec2::splat(32.0)),
                    ..default()
                },
                Transform::from_translation(current_pos),
            )).id();
            
            trail.segment_entities.push(segment);
        }
        
        // Limit trail length
        if trail.positions.len() > trail.max_length {
            trail.positions.remove(0);
            if let Some(old_segment) = trail.segment_entities.first() {
                commands.entity(*old_segment).despawn();
                trail.segment_entities.remove(0);
            }
        }
    }
}

fn update_pulse_effects(
    mut pulse_q: Query<(&mut PulseEffect, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut pulse, mut transform, mut sprite) in pulse_q.iter_mut() {
        pulse.lifetime.tick(time.delta());
        
        if !pulse.lifetime.finished() {
            let progress = pulse.lifetime.fraction();
            
            if pulse.shockwave {
                // Expanding ring
                let scale = pulse.base_scale + progress * pulse.pulse_scale;
                transform.scale = Vec3::splat(scale);
                
                // Fade out
                let alpha = (1.0 - progress) * 0.6;
                sprite.color = get_fruit_color(pulse.fruit_type).with_alpha(alpha);
            } else {
                // Pulsing effect
                let pulse_time = time.elapsed_seconds() * pulse.pulse_speed;
                let scale_factor = pulse.base_scale + (pulse_time.sin() * 0.5 + 0.5) * pulse.pulse_scale;
                transform.scale = Vec3::splat(scale_factor);
                
                let alpha = (1.0 - pulse.lifetime.fraction()) * 0.5;
                sprite.color = get_fruit_color(pulse.fruit_type).with_alpha(alpha);
            }
        }
    }
}

fn update_aura_effects(
    mut aura_q: Query<(&mut AuraEffect, &mut Transform, &mut Sprite)>,
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<FruitVisualAssets>,
) {
    for (mut aura, mut transform, mut sprite) in aura_q.iter_mut() {
        aura.lifetime.tick(time.delta());
        aura.particle_spawn_timer.tick(time.delta());
        
        if !aura.lifetime.finished() {
            // Rotate the aura
            transform.rotate_z(aura.rotation_speed * time.delta_secs());
            
            // Pulse the size
            let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.1 + 1.0;
            transform.scale = Vec3::splat(pulse);
            
            // Fade out over time
            let alpha = (1.0 - aura.lifetime.fraction()) * 0.3;
            sprite.color = get_fruit_color(aura.fruit_type).with_alpha(alpha);
            
            // Spawn aura particles
            if aura.particle_spawn_timer.just_finished() {
                let angle = rand::random::<f32>() * std::f32::consts::TAU;
                let radius = aura.inner_radius + rand::random::<f32>() * (aura.outer_radius - aura.inner_radius);
                let pos = Vec3::new(
                    angle.cos() * radius,
                    angle.sin() * radius,
                    1.0,
                );
                
                spawn_aura_particle(&mut commands, transform.translation + pos, aura.fruit_type, &assets);
            }
        }
    }
}

fn spawn_aura_particle(commands: &mut Commands, position: Vec3, fruit_type: u8, assets: &FruitVisualAssets) {
    commands.spawn((
        ParticleData {
            position: position.truncate(),
            velocity: Vec2::new(0.0, 20.0),
            lifetime: 1.0,
            size: 8.0,
            rotation: 0.0,
            particle_type: fruit_type as usize * 8,
        },
        Sprite {
            image: assets.particle_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.particle_atlas.clone(),
                index: fruit_type as usize * 8,
            }),
            color: get_fruit_color(fruit_type).with_alpha(0.5),
            custom_size: Some(Vec2::splat(8.0)),
            ..default()
        },
        Transform::from_translation(position),
    ));
}

fn update_chromatic_aberration(
    mut chroma_q: Query<(Entity, &mut ChromaticAberration)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Handle chromatic aberration (would need post-processing setup)
    for (entity, mut chroma) in chroma_q.iter_mut() {
        chroma.duration.tick(time.delta());
        
        if chroma.duration.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_expired_visuals(
    mut commands: Commands,
    particle_q: Query<(Entity, &ParticleSystem)>,
    pulse_q: Query<(Entity, &PulseEffect)>,
    aura_q: Query<(Entity, &AuraEffect)>,
    trail_segment_q: Query<(Entity, &TrailSegment)>,
) {
    // Cleanup particles
    for (entity, system) in particle_q.iter() {
        if system.lifetime.finished() && system.particles.is_empty() {
            commands.entity(entity).despawn();
        }
    }
    
    // Cleanup pulse effects
    for (entity, pulse) in pulse_q.iter() {
        if pulse.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
    
    // Cleanup aura effects
    for (entity, aura) in aura_q.iter() {
        if aura.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
    
    // Cleanup trail segments
    for (entity, segment) in trail_segment_q.iter() {
        if segment.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
