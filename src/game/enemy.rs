use bevy::prelude::*;
use crate::game::animation::{AnimationController, AnimationClip};
use crate::game::combat::{Health, CombatStats};
use crate::game::movement::{Velocity, Collider};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEnemyEvent>()
            .add_event::<SpawnBossEvent>()
            .init_resource::<EnemyAssets>()
            .add_systems(Startup, load_enemy_assets)
            .add_systems(Update, (
                handle_spawn_events,
                enemy_ai_system,
                update_enemy_behavior,
            ));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub ai_state: AIState,
    pub detection_range: f32,
    pub attack_range: f32,
    pub patrol_origin: Vec2,
    pub behavior_timer: Timer,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    // Basic enemies
    Goblin,
    Skeleton,
    Orc,
    // Advanced enemies
    DarkKnight,
    Necromancer,
    // Bosses
    GoblinKing,
    LichLord,
    DragonKnight,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AIState {
    Idle,
    Patrolling,
    Chasing,
    Attacking,
    Fleeing,
    Stunned,
}

#[derive(Component)]
pub struct Boss {
    pub phase: u8,
    pub enrage_timer: Timer,
}

#[derive(Event)]
pub struct SpawnEnemyEvent {
    pub position: Vec3,
    pub enemy_type: EnemyType,
}

#[derive(Event)]
pub struct SpawnBossEvent {
    pub position: Vec3,
    pub boss_type: EnemyType,
}

#[derive(Resource, Default)]
pub struct EnemyAssets {
    pub textures: Vec<Handle<Image>>,
    pub layouts: Vec<Handle<TextureAtlasLayout>>,
}

fn load_enemy_assets(
    mut enemy_assets: ResMut<EnemyAssets>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load textures
    enemy_assets.textures.push(asset_server.load("sprites/player_x.png"));
    
    // Create layouts
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4, 2,
        None, None,
    );
    enemy_assets.layouts.push(layouts.add(layout));
}

fn handle_spawn_events(
    mut commands: Commands,
    mut enemy_events: EventReader<SpawnEnemyEvent>,
    mut boss_events: EventReader<SpawnBossEvent>,
    enemy_assets: Res<EnemyAssets>,
) {
    for event in enemy_events.read() {
        spawn_enemy(&mut commands, &enemy_assets, event.position, event.enemy_type);
    }
    
    for event in boss_events.read() {
        spawn_boss(&mut commands, &enemy_assets, event.position, event.boss_type);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    assets: &EnemyAssets,
    position: Vec3,
    enemy_type: EnemyType,
) {
    let (health, damage, armor, speed, color) = match enemy_type {
        EnemyType::Goblin => (30, 5, 0, 100.0, Color::linear_rgb(0.0, 0.8, 0.0)),
        EnemyType::Skeleton => (50, 8, 2, 75.0, Color::linear_rgb(0.9, 0.9, 0.9)),
        EnemyType::Orc => (80, 12, 5, 50.0, Color::linear_rgb(0.6, 0.4, 0.0)),
        EnemyType::DarkKnight => (150, 20, 10, 60.0, Color::linear_rgb(0.2, 0.0, 0.2)),
        EnemyType::Necromancer => (100, 15, 3, 40.0, Color::linear_rgb(0.5, 0.0, 0.5)),
        _ => (100, 10, 5, 50.0, Color::linear_rgb(1.0, 1.0, 1.0)),
    };

    let mut anim = AnimationController::new();
    anim.add_animation("idle", AnimationClip::new(0, 3, 0.3, true));
    anim.add_animation("walk", AnimationClip::new(4, 7, 0.15, true));
    anim.play("idle");

    commands.spawn((
        Enemy {
            enemy_type,
            ai_state: AIState::Idle,
            detection_range: 300.0,
            attack_range: 40.0,
            patrol_origin: position.truncate(),
            behavior_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        },
        Health::new(health),
        CombatStats {
            damage,
            armor,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
        },
        Velocity(Vec2::ZERO),
        Collider { size: Vec2::splat(28.0) },
        Sprite {
            image: assets.textures[0].clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.layouts[0].clone(),
                index: 0,
            }),
            color,
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform::from_translation(position),
        anim,
    ));
}

fn spawn_boss(
    commands: &mut Commands,
    assets: &EnemyAssets,
    position: Vec3,
    boss_type: EnemyType,
) {
    let (health, damage, armor) = match boss_type {
        EnemyType::GoblinKing => (500, 25, 10),
        EnemyType::LichLord => (800, 30, 15),
        EnemyType::DragonKnight => (1200, 40, 20),
        _ => (500, 20, 10),
    };

    let mut anim = AnimationController::new();
    anim.add_animation("idle", AnimationClip::new(0, 3, 0.4, true));
    anim.add_animation("walk", AnimationClip::new(4, 7, 0.2, true));
    anim.play("idle");

    commands.spawn((
        Enemy {
            enemy_type: boss_type,
            ai_state: AIState::Idle,
            detection_range: 500.0,
            attack_range: 60.0,
            patrol_origin: position.truncate(),
            behavior_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Boss {
            phase: 1,
            enrage_timer: Timer::from_seconds(180.0, TimerMode::Once),
        },
        Health::new(health),
        CombatStats {
            damage,
            armor,
            crit_chance: 0.15,
            crit_multiplier: 2.0,
        },
        Velocity(Vec2::ZERO),
        Collider { size: Vec2::splat(64.0) },
        Sprite {
            image: assets.textures[0].clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.layouts[0].clone(),
                index: 0,
            }),
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_translation(position),
        anim,
    ));
}

fn enemy_ai_system(
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity, &mut AnimationController)>,
    player_q: Query<&Transform, With<crate::game::player::Player>>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };
    
    for (mut enemy, enemy_tf, mut velocity, mut anim) in enemy_q.iter_mut() {
        enemy.behavior_timer.tick(time.delta());
        
        let to_player = player_tf.translation - enemy_tf.translation;
        let distance = to_player.length();
        
        // State machine
        match enemy.ai_state {
            AIState::Idle => {
                if distance < enemy.detection_range {
                    enemy.ai_state = AIState::Chasing;
                } else if enemy.behavior_timer.just_finished() {
                    enemy.ai_state = AIState::Patrolling;
                }
            }
            AIState::Patrolling => {
                // Simple patrol behavior
                let patrol_target = enemy.patrol_origin + Vec2::new(
                    (time.elapsed_secs() * 0.5).sin() * 100.0,
                    (time.elapsed_secs() * 0.5).cos() * 100.0,
                );
                let to_patrol = (patrol_target - enemy_tf.translation.truncate()).normalize_or_zero();
                velocity.0 = to_patrol * 30.0;
                
                if distance < enemy.detection_range {
                    enemy.ai_state = AIState::Chasing;
                }
                
                if anim.current != "walk" {
                    anim.play("walk");
                }
            }
            AIState::Chasing => {
                if distance < enemy.attack_range {
                    enemy.ai_state = AIState::Attacking;
                    velocity.0 = Vec2::ZERO;
                } else if distance > enemy.detection_range * 1.5 {
                    enemy.ai_state = AIState::Idle;
                    velocity.0 = Vec2::ZERO;
                } else {
                    let direction = to_player.truncate().normalize_or_zero();
                    velocity.0 = direction * 75.0;
                    if anim.current != "walk" {
                        anim.play("walk");
                    }
                }
            }
            AIState::Attacking => {
                velocity.0 = Vec2::ZERO;
                if distance > enemy.attack_range {
                    enemy.ai_state = AIState::Chasing;
                }
                // Attack logic handled in combat system
            }
            AIState::Fleeing => {
                let direction = -to_player.truncate().normalize_or_zero();
                velocity.0 = direction * 100.0;
                if distance > enemy.detection_range {
                    enemy.ai_state = AIState::Idle;
                }
            }
            AIState::Stunned => {
                velocity.0 = Vec2::ZERO;
                if enemy.behavior_timer.just_finished() {
                    enemy.ai_state = AIState::Idle;
                }
            }
        }
        
        // Stop animation when idle
        if velocity.0.length() < 0.1 && anim.current == "walk" {
            anim.play("idle");
        }
    }
}

fn update_enemy_behavior(
    mut enemy_q: Query<(&mut Enemy, &Health)>,
) {
    for (mut enemy, health) in enemy_q.iter_mut() {
        // Flee when health is low
        if health.percentage() < 0.2 && enemy.ai_state != AIState::Fleeing {
            enemy.ai_state = AIState::Fleeing;
        }
    }
}
