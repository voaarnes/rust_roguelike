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
    // Strawberry Head: Rapid Fire - High-speed projectile barrage for consistent DPS
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
    
    // Strawberry Torso: Speed Field - Area buff that enhances mobility for tactical positioning
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
    
    // Strawberry Legs: Berry Trail - Movement-based damage for hit-and-run tactics
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
    // Pear Head: Bouncing Shot - Chain damage projectile that spreads between clustered enemies
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
    
    // Pear Torso: Healing Pulse - Sustain ability that keeps you in the fight longer
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
    
    // Pear Legs: Slippery Escape - Creates a slippery trail that slows enemies and boosts player speed
    registry.abilities.insert(
        AbilityId { fruit_type: 1, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Slippery Escape".to_string(),
            description: "Creates a slippery pear juice trail that slows enemies while boosting your speed".to_string(),
            cooldown: 4.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 2,
                radius: 40.0,
                tick_rate: 0.3,
                duration: 4.0,
                effect_type: AreaEffectType::SlowField,
            }),
            visual_effect: VisualEffectType::Trail,
        }
    );
}

fn register_mango_abilities(registry: &mut AbilityRegistry) {
    // Mango Head: Explosive Shot - High-damage burst projectile for taking down tough enemies
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
    
    // Mango Torso: Burning Aura - Continuous area damage for crowd control
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
    
    // Mango Legs: Molten Step - Each step creates a burning area that damages enemies over time
    registry.abilities.insert(
        AbilityId { fruit_type: 2, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Molten Step".to_string(),
            description: "Your footsteps leave burning mango pools that damage enemies over time".to_string(),
            cooldown: 1.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 12,
                radius: 50.0,
                tick_rate: 0.5,
                duration: 3.0,
                effect_type: AreaEffectType::BurnGround,
            }),
            visual_effect: VisualEffectType::Particles(ParticleConfig {
                color: Color::srgb(1.0, 0.3, 0.0),
                count: 8,
                spread: 25.0,
                lifetime: 0.8,
            }),
        }
    );
}

fn register_pineapple_abilities(registry: &mut AbilityRegistry) {
    // Pineapple Head: Spike Volley - Multi-directional attack for handling swarms
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
    
    // Pineapple Torso: Spike Shield - Defensive summon that provides protection and damage
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
    
    // Pineapple Legs: Spike Dash - Dashes forward while dealing damage and leaving spikes behind
    registry.abilities.insert(
        AbilityId { fruit_type: 3, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Spike Dash".to_string(),
            description: "Dash forward with incredible speed, dealing damage and leaving sharp spikes in your wake".to_string(),
            cooldown: 5.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 20,
                radius: 35.0,
                tick_rate: 0.2,
                duration: 2.5,
                effect_type: AreaEffectType::BurnGround,
            }),
            visual_effect: VisualEffectType::Trail,
        }
    );
}

fn register_apple_abilities(registry: &mut AbilityRegistry) {
    // Apple Head: Gravity Well - Area control ability that pulls enemies for strategic positioning
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
    
    // Apple Torso: Life Steal Aura - Survivability buff that converts damage into health
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
    
    // Apple Legs: Gravity Jump - Launches into the air and slams down with gravitational force
    registry.abilities.insert(
        AbilityId { fruit_type: 4, body_part: BodyPart::Legs },
        AbilityDefinition {
            name: "Gravity Slam".to_string(),
            description: "Jump high and slam down with gravitational force, creating a damaging shockwave".to_string(),
            cooldown: 6.0,
            ability_type: AbilityType::AreaEffect(AreaEffectConfig {
                damage: 45,
                radius: 120.0,
                tick_rate: 1.0,
                duration: 0.2,
                effect_type: AreaEffectType::Explosion,
            }),
            visual_effect: VisualEffectType::Pulse,
        }
    );
}

fn register_carrot_abilities(registry: &mut AbilityRegistry) {
    // Carrot Head: Piercing Lance - Long-range penetrating attack for clearing enemy lines
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
    
    // Carrot Torso: Root Spikes - Ground-based area denial that controls enemy movement
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
    
    // Carrot Legs: Burrow - Emergency defensive ability that provides temporary invulnerability
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
    // Coconut Head: Coconut Cannon - Heavy-hitting slow projectile for high single-target damage
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
    
    // Coconut Torso: Hard Shell - Tank-focused defensive buff for sustained combat
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
    
    // Coconut Legs: Earthquake - Massive area damage ability for clearing large groups
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
