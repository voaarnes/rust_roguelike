use super::*;

pub fn register_all_abilities(registry: &mut AbilityRegistry) {
    // Strawberry (0) abilities
    register_strawberry_abilities(registry);
    
    // Pear (1) abilities
    register_pear_abilities(registry);
    
    // Mango (2) abilities
    register_mango_abilities(registry);
    
    // Pineapple (3) abilities
    register_pineapple_abilities(registry);
    
    // Apple (4) abilities
    register_apple_abilities(registry);
    
    // Carrot (5) abilities
    register_carrot_abilities(registry);
    
    // Coconut (6) abilities
    register_coconut_abilities(registry);
}

fn register_strawberry_abilities(registry: &mut AbilityRegistry) {
    // Strawberry Head: Rapid Fire - Shoots fast projectiles at nearest enemy
    registry.abilities.insert(
        AbilityId { fruit_type: 0, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Rapid Fire".to_string(),
            description: "Shoots rapid strawberry seeds at the nearest enemy".to_string(),
            cooldown: 0.5,
            ability_type: AbilityType::Projectile(ProjectileConfig {
                damage: 5,
                speed: 500.0,
                pierce_count: 0,
                targeting: TargetingType::Nearest,
                projectile_visual: ProjectileVisual::Strawberry,
            }),
            visual_effect: VisualEffectType::Trail,
        }
    );
    
    // Strawberry Torso: Speed Aura - Increases movement speed in an area
    registry.abilities.insert(
        AbilityId { fruit_type: 0, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Speed Field".to_string(),
            description: "Creates a speed boost field around the player".to_string(),
            cooldown: 10.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 0,
                radius: 150.0,
                tick_rate: 0.5,
                duration: 5.0,
                effect_type: AreaEffectType::SlowField, // Actually speeds up allies
            }),
            visual_effect: VisualEffectType::Aura,
        }
    );
    
    // Strawberry Legs: Dash Trail - Leaves damaging trail when moving
    registry.abilities.insert(
        AbilityId { fruit_type: 0, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Berry Trail".to_string(),
            description: "Leaves a trail of strawberry juice that damages enemies".to_string(),
            cooldown: 2.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 3,
                radius: 30.0,
                tick_rate: 0.2,
                duration: 3.0,
                effect_type: AreaEffectType::BurnGround,
            }),
            visual_effect: VisualEffectType::Trail,
        }
    );
}

fn register_pear_abilities(registry: &mut AbilityRegistry) {
    // Pear Head: Bouncing Shot - Projectile that bounces between enemies
    registry.abilities.insert(
        AbilityId { fruit_type: 1, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Bouncing Pear".to_string(),
            description: "Shoots a pear that bounces between enemies".to_string(),
            cooldown: 2.0,
            ability_type: AbilityType::Projectile(ProjectileConfig {
                damage: 15,
                speed: 300.0,
                pierce_count: 3,
                targeting: TargetingType::Nearest,
                projectile_visual: ProjectileVisual::Pear,
            }),
            visual_effect: VisualEffectType::Particles(ParticleConfig {
                color: Color::srgb(0.7, 1.0, 0.3),
                count: 5,
                spread: 20.0,
                lifetime: 0.5,
            }),
        }
    );
    
    // Pear Torso: Healing Pulse
    registry.abilities.insert(
        AbilityId { fruit_type: 1, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Healing Pulse".to_string(),
            description: "Periodically heals the player".to_string(),
            cooldown: 5.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: -10, // Negative damage = healing
                radius: 50.0,
                tick_rate: 1.0,
                duration: 0.1,
                effect_type: AreaEffectType::HealingAura,
            }),
            visual_effect: VisualEffectType::Pulse,
        }
    );
    
    // Pear Legs: Placeholder
    registry.abilities.insert(
        AbilityId { fruit_type: 1, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Pear Slide".to_string(),
            description: "Placeholder ability".to_string(),
            cooldown: 3.0,
            ability_type: AbilityType::Buff(BuffConfig {
                stat_modifier: StatModifier::SpeedBoost(1.5),
                duration: 2.0,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
}

fn register_mango_abilities(registry: &mut AbilityRegistry) {
    // Mango Head: Explosive Shot
    registry.abilities.insert(
        AbilityId { fruit_type: 2, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Mango Bomb".to_string(),
            description: "Launches an explosive mango".to_string(),
            cooldown: 3.0,
            ability_type: AbilityType::Projectile(ProjectileConfig {
                damage: 30,
                speed: 250.0,
                pierce_count: 0,
                targeting: TargetingType::Nearest,
                projectile_visual: ProjectileVisual::Mango,
            }),
            visual_effect: VisualEffectType::Particles(ParticleConfig {
                color: Color::srgb(1.0, 0.6, 0.0),
                count: 20,
                spread: 50.0,
                lifetime: 1.0,
            }),
        }
    );
    
    // Mango Torso: Fire Aura - Damages enemies around player
    registry.abilities.insert(
        AbilityId { fruit_type: 2, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Burning Aura".to_string(),
            description: "Burns nearby enemies continuously".to_string(),
            cooldown: 1.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 8,
                radius: 100.0,
                tick_rate: 0.5,
                duration: 0.5,
                effect_type: AreaEffectType::Explosion,
            }),
            visual_effect: VisualEffectType::Aura,
        }
    );
    
    // Mango Legs: Placeholder
    registry.abilities.insert(
        AbilityId { fruit_type: 2, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Mango Stomp".to_string(),
            description: "Placeholder ability".to_string(),
            cooldown: 4.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 25,
                radius: 80.0,
                tick_rate: 1.0,
                duration: 0.1,
                effect_type: AreaEffectType::Explosion,
            }),
            visual_effect: VisualEffectType::Pulse,
        }
    );
}

fn register_pineapple_abilities(registry: &mut AbilityRegistry) {
    // Pineapple Head: Spike Volley - Shoots spikes in all directions
    registry.abilities.insert(
        AbilityId { fruit_type: 3, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Spike Volley".to_string(),
            description: "Shoots pineapple spikes in all directions".to_string(),
            cooldown: 2.5,
            ability_type: AbilityType::Projectile(ProjectileConfig {
                damage: 10,
                speed: 400.0,
                pierce_count: 1,
                targeting: TargetingType::AllDirections,
                projectile_visual: ProjectileVisual::Pineapple,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
    
    // Pineapple Torso: Spike Shield
    registry.abilities.insert(
        AbilityId { fruit_type: 3, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Spike Shield".to_string(),
            description: "Summons rotating spike shields".to_string(),
            cooldown: 8.0,
            ability_type: AbilityType::Summon(SummonConfig {
                summon_type: SummonType::Shield,
                duration: 5.0,
                count: 3,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
    
    // Pineapple Legs: Placeholder
    registry.abilities.insert(
        AbilityId { fruit_type: 3, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Pineapple Roll".to_string(),
            description: "Placeholder ability".to_string(),
            cooldown: 3.0,
            ability_type: AbilityType::Buff(BuffConfig {
                stat_modifier: StatModifier::ArmorBoost(10),
                duration: 3.0,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
}

fn register_apple_abilities(registry: &mut AbilityRegistry) {
    // Apple Head: Gravity Well - Pulls enemies and damages
    registry.abilities.insert(
        AbilityId { fruit_type: 4, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Newton's Force".to_string(),
            description: "Creates a gravity well that pulls enemies".to_string(),
            cooldown: 4.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 5,
                radius: 150.0,
                tick_rate: 0.2,
                duration: 2.0,
                effect_type: AreaEffectType::SlowField,
            }),
            visual_effect: VisualEffectType::Pulse,
        }
    );
    
    // Apple Torso: Life Steal Aura
    registry.abilities.insert(
        AbilityId { fruit_type: 4, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Life Steal".to_string(),
            description: "Gain health from damage dealt".to_string(),
            cooldown: 10.0,
            ability_type: AbilityType::Buff(BuffConfig {
                stat_modifier: StatModifier::LifeSteal(0.3),
                duration: 5.0,
            }),
            visual_effect: VisualEffectType::Aura,
        }
    );
    
    // Apple Legs: Placeholder
    registry.abilities.insert(
        AbilityId { fruit_type: 4, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Apple Bounce".to_string(),
            description: "Placeholder ability".to_string(),
            cooldown: 2.0,
            ability_type: AbilityType::Buff(BuffConfig {
                stat_modifier: StatModifier::SpeedBoost(1.2),
                duration: 1.5,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
}

fn register_carrot_abilities(registry: &mut AbilityRegistry) {
    // Carrot Head: Piercing Lance
    registry.abilities.insert(
        AbilityId { fruit_type: 5, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Carrot Lance".to_string(),
            description: "Shoots a piercing carrot lance".to_string(),
            cooldown: 1.5,
            ability_type: AbilityType::Projectile(ProjectileConfig {
                damage: 20,
                speed: 600.0,
                pierce_count: 5,
                targeting: TargetingType::Forward,
                projectile_visual: ProjectileVisual::Carrot,
            }),
            visual_effect: VisualEffectType::Trail,
        }
    );
    
    // Carrot Torso: Underground Spikes
    registry.abilities.insert(
        AbilityId { fruit_type: 5, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Root Spikes".to_string(),
            description: "Summons spikes from the ground".to_string(),
            cooldown: 3.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 15,
                radius: 120.0,
                tick_rate: 0.5,
                duration: 2.0,
                effect_type: AreaEffectType::BurnGround,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
    
    // Carrot Legs: Burrow
    registry.abilities.insert(
        AbilityId { fruit_type: 5, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Burrow".to_string(),
            description: "Briefly become invulnerable".to_string(),
            cooldown: 6.0,
            ability_type: AbilityType::Buff(BuffConfig {
                stat_modifier: StatModifier::ArmorBoost(100),
                duration: 1.0,
            }),
            visual_effect: VisualEffectType::None,
        }
    );
}

fn register_coconut_abilities(registry: &mut AbilityRegistry) {
    // Coconut Head: Bouncing Coconut
    registry.abilities.insert(
        AbilityId { fruit_type: 6, body_part: BodyPart::Head },
        AbilityDefinition {
            name: "Coconut Cannon".to_string(),
            description: "Launches heavy coconuts that stun".to_string(),
            cooldown: 3.5,
            ability_type: AbilityType::Projectile(ProjectileConfig {
                damage: 40,
                speed: 200.0,
                pierce_count: 0,
                targeting: TargetingType::Nearest,
                projectile_visual: ProjectileVisual::Coconut,
            }),
            visual_effect: VisualEffectType::Particles(ParticleConfig {
                color: Color::srgb(0.6, 0.4, 0.2),
                count: 10,
                spread: 30.0,
                lifetime: 0.8,
            }),
        }
    );
    
    // Coconut Torso: Hard Shell - Damage reduction
    registry.abilities.insert(
        AbilityId { fruit_type: 6, body_part: BodyPart::Torso },
        AbilityDefinition {
            name: "Hard Shell".to_string(),
            description: "Greatly increases armor".to_string(),
            cooldown: 12.0,
            ability_type: AbilityType::Buff(BuffConfig {
                stat_modifier: StatModifier::ArmorBoost(20),
                duration: 6.0,
            }),
            visual_effect: VisualEffectType::Aura,
        }
    );
    
    // Coconut Legs: Earthquake
    registry.abilities.insert(
        AbilityId { fruit_type: 6, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Earthquake".to_string(),
            description: "Creates damaging shockwaves".to_string(),
            cooldown: 5.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 35,
                radius: 200.0,
                tick_rate: 1.0,
                duration: 0.1,
                effect_type: AreaEffectType::Explosion,
            }),
            visual_effect: VisualEffectType::Pulse,
        }
    );
}
