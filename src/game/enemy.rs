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
    pub move_speed: f32,
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
    // Load textures - using fallback texture for now
    enemy_assets.textures.push(asset_server.load("sprites/test_p_sprite.png"));
    
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
        EnemyType::Goblin => (30, 5, 0, 80.0, Color::linear_rgb(0.0, 0.8, 0.0)),
        EnemyType::Skeleton => (50, 8, 2, 60.0, Color::linear_rgb(0.9, 0.9, 0.9)),
        EnemyType::Orc => (80, 12, 5, 40.0, Color::linear_rgb(0.6, 0.4, 0.0)),
        EnemyType::DarkKnight => (150, 20, 10, 50.0, Color::linear_rgb(0.2, 0.0, 0.2)),
        EnemyType::Necromancer => (100, 15, 3, 30.0, Color::linear_rgb(0.5, 0.0, 0.5)),
        _ => (100, 10, 5, 50.0, Color::linear_rgb(1.0, 1.0, 1.0)),
    };

    let mut anim = AnimationController::new();
    anim.add_animation("idle", AnimationClip::new(0, 3, 0.3, true));
    anim.add_animation("walk", AnimationClip::new(4, 7, 0.15, true));
    anim.play("idle");

    if let (Some(texture), Some(layout)) = (assets.textures.first(), assets.layouts.first()) {
        commands.spawn((
            Enemy {
                enemy_type,
                ai_state: AIState::Idle,
                detection_range: 300.0,
                attack_range: 40.0,
                patrol_origin: position.truncate(),
                behavior_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                move_speed: speed,
            },
            Health::new(health),
            CombatStats {
                damage,
                armor,
                crit_chance: 0.1,
                crit_multiplier: 1.5,
            },
            Velocity(Vec2::ZERO),
            Collider { size: Vec2::splat(28.0) },
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout.clone(),
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
}

fn spawn_boss(
    commands: &mut Commands,
    assets: &EnemyAssets,
    position: Vec3,
    boss_type: EnemyType,
) {
    let (health, damage, armor, speed, color) = match boss_type {
        EnemyType::GoblinKing => (500, 25, 5, 30.0, Color::linear_rgb(0.0, 1.0, 0.0)),
        EnemyType::LichLord => (800, 40, 10, 20.0, Color::linear_rgb(0.5, 0.0, 1.0)),
        EnemyType::DragonKnight => (1200, 60, 15, 25.0, Color::linear_rgb(1.0, 0.0, 0.0)),
        _ => (500, 25, 5, 30.0, Color::linear_rgb(1.0, 1.0, 1.0)),
    };

    let mut anim = AnimationController::new();
    anim.add_animation("idle", AnimationClip::new(0, 3, 0.3, true));
    anim.add_animation("walk", AnimationClip::new(4, 7, 0.15, true));
    anim.play("idle");

    if let (Some(texture), Some(layout)) = (assets.textures.first(), assets.layouts.first()) {
        commands.spawn((
            Enemy {
                enemy_type: boss_type,
                ai_state: AIState::Idle,
                detection_range: 500.0,
                attack_range: 60.0,
                patrol_origin: position.truncate(),
                behavior_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                move_speed: speed,
            },
            Boss {
                phase: 1,
                enrage_timer: Timer::from_seconds(10.0, TimerMode::Once),
            },
            Health::new(health),
            CombatStats {
                damage,
                armor,
                crit_chance: 0.2,
                crit_multiplier: 2.0,
            },
            Velocity(Vec2::ZERO),
            Collider { size: Vec2::splat(48.0) },
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout.clone(),
                    index: 0,
                }),
                color,
                custom_size: Some(Vec2::splat(64.0)),
                ..default()
            },
            Transform::from_translation(position),
            anim,
        ));
    }
}

fn enemy_ai_system(
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity, &mut AnimationController)>,
    player_q: Query<&Transform, (With<crate::game::player::Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.single() else { return };
    
    for (mut enemy, enemy_tf, mut velocity, mut anim) in enemy_q.iter_mut() {
        enemy.behavior_timer.tick(time.delta());
        
        let to_player = player_tf.translation - enemy_tf.translation;
        let distance = to_player.length();
        
        // State machine
        match enemy.ai_state {
            AIState::Idle => {
                velocity.0 = Vec2::ZERO;
                if distance < enemy.detection_range {
                    enemy.ai_state = AIState::Chasing;
                }
                if anim.current != "idle" {
                    anim.play("idle");
                }
            }
            AIState::Chasing => {
                if distance > enemy.detection_range * 1.5 {
                    enemy.ai_state = AIState::Idle;
                } else {
                    // Move towards player
                    let direction = to_player.truncate().normalize_or_zero();
                    velocity.0 = direction * enemy.move_speed;
                    
                    if anim.current != "walk" {
                        anim.play("walk");
                    }
                }
            }
            _ => {
                velocity.0 = Vec2::ZERO;
            }
        }
    }
}

fn update_enemy_behavior(
    mut boss_q: Query<(&mut Boss, &mut Enemy, &Health)>,
    _time: Res<Time>,
) {
    for (mut boss, mut enemy, health) in boss_q.iter_mut() {
        // Boss gets more aggressive at low health - enter phase 2
        if health.percentage() < 0.3 && boss.phase == 1 {
            boss.phase = 2;
            enemy.move_speed *= 1.5;
        }
    }
}
