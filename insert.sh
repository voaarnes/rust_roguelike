#!/bin/bash

# Create improved directory structure
mkdir -p src/core
mkdir -p src/game
mkdir -p src/game/combat
mkdir -p src/game/movement
mkdir -p src/game/spawning
mkdir -p src/game/progression
mkdir -p src/ui
mkdir -p src/world
mkdir -p src/world/generation
mkdir -p src/utils
mkdir -p assets/data
mkdir -p assets/levels
mkdir -p assets/config

# Remove old structure
rm -rf src/systems
rm -rf src/components
rm -rf src/resources
rm -rf src/entities
rm -rf src/tilemap
rm -rf src/animation
rm -rf src/audio
rm -f src/setup.rs
rm -f src/constants.rs

# ==============================================
# MAIN AND CORE FILES
# ==============================================

# Create main.rs with improved plugin organization
cat > src/main.rs << 'EOF'
mod core;
mod game;
mod ui;
mod world;
mod utils;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        // Configure window and renderer
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Roguelike - Survivor".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        // Development diagnostics (remove in release)
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        // Core game plugins
        .add_plugins((
            core::CorePlugin,
            game::GamePlugin,
            ui::UIPlugin,
            world::WorldPlugin,
            utils::UtilsPlugin,
        ))
        .run();
}
EOF

# Create core/mod.rs - Core systems and resources
cat > src/core/mod.rs << 'EOF'
pub mod state;
pub mod config;
pub mod events;
pub mod camera;
pub mod input;
pub mod save_system;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add game states
            .init_state::<state::GameState>()
            .init_state::<state::PlayState>()
            // Resources
            .init_resource::<config::GameConfig>()
            .init_resource::<input::InputBuffer>()
            .init_resource::<save_system::SaveData>()
            // Events
            .add_event::<events::GameEvent>()
            .add_event::<events::PlayerEvent>()
            .add_event::<events::CombatEvent>()
            // Systems
            .add_systems(Startup, camera::setup_camera)
            .add_systems(Update, (
                input::buffer_input_system,
                camera::camera_follow_player.run_if(in_state(state::GameState::Playing)),
                save_system::auto_save_system,
            ));
    }
}
EOF

# Create core/state.rs - Game state management
cat > src/core/state.rs << 'EOF'
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayState {
    #[default]
    Exploring,
    Combat,
    Shopping,
    Dialogue,
    Transitioning,
}

#[derive(Resource)]
pub struct GameStats {
    pub current_level: usize,
    pub score: u32,
    pub enemies_killed: u32,
    pub time_played: f32,
    pub coins_collected: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub distance_traveled: f32,
    pub abilities_used: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            current_level: 1,
            score: 0,
            enemies_killed: 0,
            time_played: 0.0,
            coins_collected: 0,
            damage_dealt: 0,
            damage_taken: 0,
            distance_traveled: 0.0,
            abilities_used: 0,
        }
    }
}
EOF

# Create core/config.rs - Game configuration
cat > src/core/config.rs << 'EOF'
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    // Player settings
    pub player_base_speed: f32,
    pub player_base_health: i32,
    pub player_base_damage: i32,
    
    // Enemy settings
    pub enemy_spawn_rate: f32,
    pub enemy_difficulty_scaling: f32,
    pub boss_spawn_time: f32,
    
    // World settings
    pub tile_size: f32,
    pub chunk_size: u32,
    
    // UI settings
    pub show_health_bars: bool,
    pub show_damage_numbers: bool,
    pub show_minimap: bool,
    
    // Audio settings
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    
    // Graphics settings
    pub particle_effects: bool,
    pub screen_shake: bool,
    pub vsync: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_base_speed: 200.0,
            player_base_health: 100,
            player_base_damage: 10,
            enemy_spawn_rate: 3.0,
            enemy_difficulty_scaling: 1.1,
            boss_spawn_time: 600.0,
            tile_size: 32.0,
            chunk_size: 16,
            show_health_bars: true,
            show_damage_numbers: true,
            show_minimap: true,
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            particle_effects: true,
            screen_shake: true,
            vsync: true,
        }
    }
}

impl GameConfig {
    pub fn load() -> Self {
        // Try to load from file, otherwise use defaults
        if let Ok(contents) = std::fs::read_to_string("config.json") {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }
    
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write("config.json", json);
        }
    }
}
EOF

# Create core/events.rs - Central event system
cat > src/core/events.rs << 'EOF'
use bevy::prelude::*;

#[derive(Event)]
pub enum GameEvent {
    LevelCompleted { level: usize },
    BossDefeated { boss_type: String },
    AchievementUnlocked { achievement_id: String },
    QuestCompleted { quest_id: String },
}

#[derive(Event)]
pub enum PlayerEvent {
    LevelUp { new_level: u32 },
    SkillUnlocked { skill_id: String },
    ItemPickup { item_id: String, quantity: u32 },
    Death,
}

#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
    pub damage_type: DamageType,
    pub position: Vec3,
}

#[derive(Clone, Copy)]
pub enum DamageType {
    Physical,
    Magic,
    Fire,
    Ice,
    Poison,
    True, // Ignores armor
}
EOF

# Create core/camera.rs - Enhanced camera system
cat > src/core/camera.rs << 'EOF'
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera {
    pub smoothing: f32,
    pub offset: Vec2,
    pub bounds: Option<Rect>,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            smoothing: 5.0,
            offset: Vec2::ZERO,
            bounds: None,
        }
    }
}

#[derive(Component)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: Timer,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera::default(),
    ));
}

pub fn camera_follow_player(
    player_q: Query<&Transform, (With<crate::game::player::Player>, Without<MainCamera>)>,
    mut cam_q: Query<(&mut Transform, &MainCamera), Without<crate::game::player::Player>>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };
    let Ok((mut cam_tf, cam)) = cam_q.get_single_mut() else { return };
    
    let target = player_tf.translation.truncate() + cam.offset;
    let current = cam_tf.translation.truncate();
    
    let new_pos = current.lerp(target, cam.smoothing * time.delta_secs());
    
    // Apply bounds if they exist
    let final_pos = if let Some(bounds) = cam.bounds {
        Vec2::new(
            new_pos.x.clamp(bounds.min.x, bounds.max.x),
            new_pos.y.clamp(bounds.min.y, bounds.max.y),
        )
    } else {
        new_pos
    };
    
    cam_tf.translation.x = final_pos.x;
    cam_tf.translation.y = final_pos.y;
}

pub fn camera_shake_system(
    mut cam_q: Query<(&mut Transform, &mut CameraShake)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, mut shake) in cam_q.iter_mut() {
        shake.duration.tick(time.delta());
        
        if !shake.duration.finished() {
            let progress = shake.duration.fraction_remaining();
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * shake.intensity * progress,
                (rand::random::<f32>() - 0.5) * shake.intensity * progress,
            );
            transform.translation += offset.extend(0.0);
        } else {
            commands.entity(transform.entity).remove::<CameraShake>();
        }
    }
}
EOF

# Create core/input.rs - Input buffering system
cat > src/core/input.rs << 'EOF'
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct InputBuffer {
    pub buffer: VecDeque<InputAction>,
    pub max_size: usize,
    pub buffer_time: f32,
}

#[derive(Clone, Copy)]
pub struct InputAction {
    pub action: Action,
    pub timestamp: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Move(Vec2),
    Attack,
    Dash,
    UseAbility(u8),
    Interact,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            buffer: VecDeque::with_capacity(10),
            max_size: 10,
            buffer_time: 0.2,
        }
    }
}

pub fn buffer_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut buffer: ResMut<InputBuffer>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    // Clean old inputs
    buffer.buffer.retain(|action| {
        current_time - action.timestamp < buffer.buffer_time
    });
    
    // Add new inputs
    let mut movement = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { movement.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { movement.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { movement.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { movement.x += 1.0; }
    
    if movement != Vec2::ZERO {
        buffer.buffer.push_back(InputAction {
            action: Action::Move(movement.normalize()),
            timestamp: current_time,
        });
    }
    
    if keys.just_pressed(KeyCode::Space) {
        buffer.buffer.push_back(InputAction {
            action: Action::Attack,
            timestamp: current_time,
        });
    }
    
    if keys.just_pressed(KeyCode::ShiftLeft) {
        buffer.buffer.push_back(InputAction {
            action: Action::Dash,
            timestamp: current_time,
        });
    }
    
    // Trim buffer if too large
    while buffer.buffer.len() > buffer.max_size {
        buffer.buffer.pop_front();
    }
}
EOF

# Create core/save_system.rs - Save/Load system
cat > src/core/save_system.rs << 'EOF'
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct SaveData {
    pub player_level: u32,
    pub player_experience: u32,
    pub unlocked_abilities: Vec<String>,
    pub completed_levels: Vec<usize>,
    pub total_play_time: f32,
    pub high_score: u32,
    pub achievements: Vec<String>,
}

impl SaveData {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("save.json", json)?;
        Ok(())
    }
    
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("save.json")?;
        let data = serde_json::from_str(&contents)?;
        Ok(data)
    }
}

pub fn auto_save_system(
    save_data: Res<SaveData>,
    time: Res<Time>,
    mut last_save: Local<f32>,
) {
    let current_time = time.elapsed_secs();
    if current_time - *last_save > 60.0 { // Auto-save every minute
        if let Err(e) = save_data.save() {
            warn!("Failed to auto-save: {}", e);
        } else {
            info!("Game auto-saved");
        }
        *last_save = current_time;
    }
}
EOF

# ==============================================
# GAME MODULE
# ==============================================

# Create game/mod.rs
cat > src/game/mod.rs << 'EOF'
pub mod player;
pub mod enemy;
pub mod combat;
pub mod movement;
pub mod spawning;
pub mod progression;
pub mod abilities;
pub mod items;
pub mod animation;
pub mod audio;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                player::PlayerPlugin,
                enemy::EnemyPlugin,
                combat::CombatPlugin,
                movement::MovementPlugin,
                spawning::SpawningPlugin,
                progression::ProgressionPlugin,
                abilities::AbilitiesPlugin,
                items::ItemsPlugin,
                animation::AnimationPlugin,
                audio::AudioPlugin,
            ));
    }
}
EOF

# Create game/player.rs - Enhanced player system
cat > src/game/player.rs << 'EOF'
use bevy::prelude::*;
use crate::core::input::{InputBuffer, Action};
use crate::game::animation::{AnimationController, AnimationClip};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerStats>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_input_system,
                update_player_stats,
            ));
    }
}

#[derive(Component)]
pub struct Player {
    pub level: u32,
    pub experience: u32,
    pub exp_to_next_level: u32,
}

#[derive(Component)]
pub struct PlayerController {
    pub move_speed: f32,
    pub dash_speed: f32,
    pub dash_cooldown: Timer,
    pub is_dashing: bool,
}

#[derive(Resource, Default)]
pub struct PlayerStats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub vitality: u32,
    pub luck: u32,
    pub skill_points: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
            exp_to_next_level: 100,
        }
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            dash_speed: 500.0,
            dash_cooldown: Timer::from_seconds(2.0, TimerMode::Once),
            is_dashing: false,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/test_p_sprite.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4, 4,
        None, None,
    );
    let layout_handle = layouts.add(layout);

    let mut anim_controller = AnimationController::new();
    anim_controller.add_animation("idle", AnimationClip::new(0, 3, 0.2, true));
    anim_controller.add_animation("walk", AnimationClip::new(4, 7, 0.1, true));
    anim_controller.add_animation("attack", AnimationClip::new(8, 11, 0.05, false));
    anim_controller.add_animation("dash", AnimationClip::new(12, 15, 0.05, false));
    anim_controller.play("idle");

    commands.spawn((
        Player::default(),
        PlayerController::default(),
        crate::game::combat::Health::new(100),
        crate::game::combat::CombatStats {
            damage: 10,
            armor: 5,
            crit_chance: 0.1,
            crit_multiplier: 2.0,
        },
        crate::game::movement::Velocity(Vec2::ZERO),
        crate::game::movement::Collider { size: Vec2::splat(28.0) },
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        anim_controller,
    ));
}

fn player_input_system(
    mut player_q: Query<(&mut crate::game::movement::Velocity, &mut PlayerController, &mut AnimationController), With<Player>>,
    input: Res<InputBuffer>,
    time: Res<Time>,
) {
    for (mut velocity, mut controller, mut anim) in player_q.iter_mut() {
        controller.dash_cooldown.tick(time.delta());
        
        // Process buffered inputs
        for input_action in input.buffer.iter() {
            match input_action.action {
                Action::Move(dir) => {
                    if !controller.is_dashing {
                        velocity.0 = dir * controller.move_speed;
                        if anim.current != "walk" && dir.length() > 0.0 {
                            anim.play("walk");
                        }
                    }
                }
                Action::Dash => {
                    if controller.dash_cooldown.finished() && !controller.is_dashing {
                        controller.is_dashing = true;
                        controller.dash_cooldown.reset();
                        velocity.0 *= 2.5;
                        anim.play("dash");
                    }
                }
                Action::Attack => {
                    anim.play("attack");
                }
                _ => {}
            }
        }
        
        // Stop dashing
        if controller.is_dashing && anim.is_finished() {
            controller.is_dashing = false;
        }
        
        // Return to idle if not moving
        if velocity.0.length() < 0.1 && anim.current == "walk" {
            anim.play("idle");
        }
    }
}

fn update_player_stats(
    player_q: Query<&Player>,
    mut stats: ResMut<PlayerStats>,
) {
    // Update stats based on level, equipment, etc.
    for player in player_q.iter() {
        // Calculate stat bonuses
        let level_bonus = player.level as u32;
        // Apply bonuses to stats...
    }
}
EOF

# Create game/enemy.rs - Enhanced enemy system
cat > src/game/enemy.rs << 'EOF'
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
EOF

# Create game/combat/mod.rs
cat > src/game/combat/mod.rs << 'EOF'
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
                combat_events.send(CombatEvent {
                    attacker: enemy_entity,
                    target: player_entity,
                    damage: enemy_stats.damage,
                    damage_type: DamageType::Physical,
                    position: player_tf.translation,
                });
                
                // Player damages enemy (simplified melee)
                combat_events.send(CombatEvent {
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
            commands.entity(entity).despawn_recursive();
        }
    }
}
EOF

# Create game/combat/damage.rs
cat > src/game/combat/damage.rs << 'EOF'
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
EOF

# Create game/combat/effects.rs
cat > src/game/combat/effects.rs << 'EOF'
use bevy::prelude::*;

#[derive(Component)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: Timer,
    pub tick_timer: Timer,
    pub stacks: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatusEffectType {
    Poison { damage_per_tick: i32 },
    Burn { damage_per_tick: i32 },
    Freeze { slow_percentage: f32 },
    Stun,
    Regeneration { heal_per_tick: i32 },
    Shield { amount: i32 },
    SpeedBoost { multiplier: f32 },
    DamageBoost { multiplier: f32 },
}

pub fn update_status_effects(
    mut query: Query<(Entity, &mut crate::game::combat::Health, &mut StatusEffect)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut health, mut effect) in query.iter_mut() {
        effect.duration.tick(time.delta());
        effect.tick_timer.tick(time.delta());
        
        if effect.tick_timer.just_finished() {
            match effect.effect_type {
                StatusEffectType::Poison { damage_per_tick } => {
                    health.take_damage(damage_per_tick * effect.stacks as i32);
                }
                StatusEffectType::Burn { damage_per_tick } => {
                    health.take_damage(damage_per_tick * effect.stacks as i32);
                }
                StatusEffectType::Regeneration { heal_per_tick } => {
                    health.heal(heal_per_tick * effect.stacks as i32);
                }
                _ => {}
            }
            effect.tick_timer.reset();
        }
        
        if effect.duration.finished() {
            commands.entity(entity).remove::<StatusEffect>();
        }
    }
}
EOF

# Create game/combat/projectiles.rs
cat > src/game/combat/projectiles.rs << 'EOF'
use bevy::prelude::*;
use crate::core::events::{CombatEvent, DamageType};

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub damage_type: DamageType,
    pub speed: f32,
    pub direction: Vec2,
    pub lifetime: Timer,
    pub piercing: u32,
    pub owner: Entity,
}

pub fn update_projectiles(
    mut projectile_q: Query<(Entity, &mut Transform, &mut Projectile)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut projectile) in projectile_q.iter_mut() {
        projectile.lifetime.tick(time.delta());
        
        // Move projectile
        transform.translation += projectile.direction.extend(0.0) * projectile.speed * time.delta_secs();
        
        // Remove if lifetime expired
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    direction: Vec2,
    damage: i32,
    damage_type: DamageType,
    owner: Entity,
) {
    commands.spawn((
        Sprite {
            color: match damage_type {
                DamageType::Fire => Color::linear_rgb(1.0, 0.5, 0.0),
                DamageType::Ice => Color::linear_rgb(0.0, 0.5, 1.0),
                DamageType::Magic => Color::linear_rgb(0.5, 0.0, 1.0),
                _ => Color::WHITE,
            },
            custom_size: Some(Vec2::new(8.0, 8.0)),
            ..default()
        },
        Transform::from_translation(position),
        Projectile {
            damage,
            damage_type,
            speed: 300.0,
            direction: direction.normalize(),
            lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            piercing: 0,
            owner,
        },
        crate::game::movement::Collider { size: Vec2::new(8.0, 8.0) },
    ));
}
EOF

# Create game/movement/mod.rs
cat > src/game/movement/mod.rs << 'EOF'
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_velocity,
            handle_collisions,
            update_collision_grid,
        ).chain());
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Static;

#[derive(Resource, Default)]
pub struct CollisionGrid {
    pub cells: Vec<Vec<Vec<Entity>>>,
    pub cell_size: f32,
    pub width: usize,
    pub height: usize,
}

impl CollisionGrid {
    pub fn new(world_width: f32, world_height: f32, cell_size: f32) -> Self {
        let width = (world_width / cell_size).ceil() as usize;
        let height = (world_height / cell_size).ceil() as usize;
        Self {
            cells: vec![vec![Vec::new(); width]; height],
            cell_size,
            width,
            height,
        }
    }
    
    pub fn get_nearby_entities(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        let mut entities = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_x = (position.x / self.cell_size) as i32;
        let center_y = (position.y / self.cell_size) as i32;
        
        for y in (center_y - cell_radius)..=(center_y + cell_radius) {
            for x in (center_x - cell_radius)..=(center_x + cell_radius) {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    entities.extend(&self.cells[y as usize][x as usize]);
                }
            }
        }
        
        entities
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), Without<Static>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn handle_collisions(
    mut movable_q: Query<(Entity, &mut Transform, &Collider, &Velocity), Without<Static>>,
    static_q: Query<(&Transform, &Collider), With<Static>>,
    collision_grid: Res<CollisionGrid>,
) {
    for (entity, mut transform, collider, velocity) in movable_q.iter_mut() {
        let nearby = collision_grid.get_nearby_entities(transform.translation.truncate(), 100.0);
        
        for (static_tf, static_collider) in static_q.iter() {
            if check_collision(
                transform.translation.truncate(),
                collider.size,
                static_tf.translation.truncate(),
                static_collider.size,
            ) {
                // Simple push-back collision
                let diff = transform.translation - static_tf.translation;
                let overlap = (collider.size + static_collider.size) / 2.0 - diff.truncate().abs();
                
                if overlap.x > 0.0 && overlap.y > 0.0 {
                    if overlap.x < overlap.y {
                        transform.translation.x += overlap.x * diff.x.signum();
                    } else {
                        transform.translation.y += overlap.y * diff.y.signum();
                    }
                }
            }
        }
    }
}

fn update_collision_grid(
    mut grid: ResMut<CollisionGrid>,
    query: Query<(Entity, &Transform, &Collider)>,
) {
    // Clear grid
    for row in grid.cells.iter_mut() {
        for cell in row.iter_mut() {
            cell.clear();
        }
    }
    
    // Populate grid
    for (entity, transform, _) in query.iter() {
        let x = ((transform.translation.x + 1000.0) / grid.cell_size) as usize;
        let y = ((transform.translation.y + 1000.0) / grid.cell_size) as usize;
        
        if x < grid.width && y < grid.height {
            grid.cells[y][x].push(entity);
        }
    }
}

fn check_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let half1 = size1 / 2.0;
    let half2 = size2 / 2.0;
    
    (pos1.x - half1.x < pos2.x + half2.x) &&
    (pos1.x + half1.x > pos2.x - half2.x) &&
    (pos1.y - half1.y < pos2.y + half2.y) &&
    (pos1.y + half1.y > pos2.y - half2.y)
}
EOF

# Create game/spawning/mod.rs
cat > src/game/spawning/mod.rs << 'EOF'
use bevy::prelude::*;
use rand::Rng;
use crate::game::enemy::{SpawnEnemyEvent, SpawnBossEvent, EnemyType};
use crate::core::config::GameConfig;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveManager>()
            .add_systems(Update, (
                spawn_wave_system,
                update_difficulty,
            ));
    }
}

#[derive(Resource)]
pub struct WaveManager {
    pub current_wave: u32,
    pub enemies_remaining: u32,
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub difficulty_multiplier: f32,
    pub boss_spawned: bool,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_remaining: 0,
            wave_timer: Timer::from_seconds(30.0, TimerMode::Once),
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            difficulty_multiplier: 1.0,
            boss_spawned: false,
        }
    }
}

fn spawn_wave_system(
    mut wave_manager: ResMut<WaveManager>,
    mut spawn_events: EventWriter<SpawnEnemyEvent>,
    mut boss_events: EventWriter<SpawnBossEvent>,
    player_q: Query<&Transform, With<crate::game::player::Player>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    wave_manager.wave_timer.tick(time.delta());
    wave_manager.spawn_timer.tick(time.delta());
    
    // Start new wave
    if wave_manager.wave_timer.finished() && wave_manager.enemies_remaining == 0 {
        wave_manager.current_wave += 1;
        wave_manager.enemies_remaining = calculate_wave_enemies(wave_manager.current_wave);
        wave_manager.wave_timer.reset();
        
        // Boss wave every 5 waves
        if wave_manager.current_wave % 5 == 0 && !wave_manager.boss_spawned {
            wave_manager.boss_spawned = true;
            if let Ok(player_tf) = player_q.get_single() {
                boss_events.send(SpawnBossEvent {
                    position: player_tf.translation + Vec3::new(300.0, 0.0, 3.0),
                    boss_type: choose_boss_type(wave_manager.current_wave),
                });
            }
        }
    }
    
    // Spawn enemies
    if wave_manager.spawn_timer.just_finished() && wave_manager.enemies_remaining > 0 {
        if let Ok(player_tf) = player_q.get_single() {
            let mut rng = rand::thread_rng();
            let spawn_count = (3.0 * wave_manager.difficulty_multiplier) as u32;
            
            for _ in 0..spawn_count.min(wave_manager.enemies_remaining) {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(200.0..400.0);
                let spawn_pos = Vec3::new(
                    player_tf.translation.x + angle.cos() * distance,
                    player_tf.translation.y + angle.sin() * distance,
                    3.0,
                );
                
                spawn_events.send(SpawnEnemyEvent {
                    position: spawn_pos,
                    enemy_type: choose_enemy_type(wave_manager.current_wave),
                });
                
                wave_manager.enemies_remaining -= 1;
            }
        }
    }
}

fn update_difficulty(
    mut wave_manager: ResMut<WaveManager>,
    time: Res<Time>,
) {
    // Increase difficulty over time
    wave_manager.difficulty_multiplier = 1.0 + (time.elapsed_secs() / 60.0) * 0.2;
}

fn calculate_wave_enemies(wave: u32) -> u32 {
    10 + wave * 5
}

fn choose_enemy_type(wave: u32) -> EnemyType {
    let mut rng = rand::thread_rng();
    if wave < 3 {
        EnemyType::Goblin
    } else if wave < 6 {
        match rng.gen_range(0..2) {
            0 => EnemyType::Goblin,
            _ => EnemyType::Skeleton,
        }
    } else if wave < 10 {
        match rng.gen_range(0..3) {
            0 => EnemyType::Goblin,
            1 => EnemyType::Skeleton,
            _ => EnemyType::Orc,
        }
    } else {
        match rng.gen_range(0..5) {
            0 => EnemyType::Goblin,
            1 => EnemyType::Skeleton,
            2 => EnemyType::Orc,
            3 => EnemyType::DarkKnight,
            _ => EnemyType::Necromancer,
        }
    }
}

fn choose_boss_type(wave: u32) -> EnemyType {
    if wave < 10 {
        EnemyType::GoblinKing
    } else if wave < 20 {
        EnemyType::LichLord
    } else {
        EnemyType::DragonKnight
    }
}
EOF

# Create remaining game modules (simplified versions)
cat > src/game/progression/mod.rs << 'EOF'
use bevy::prelude::*;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_experience);
    }
}

fn handle_experience() {
    // Experience and leveling logic
}
EOF

cat > src/game/abilities/mod.rs << 'EOF'
use bevy::prelude::*;

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        // Ability system
    }
}
EOF

cat > src/game/items/mod.rs << 'EOF'
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // Item system
    }
}
EOF

cat > src/game/animation/mod.rs << 'EOF'
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub animations: std::collections::HashMap<String, AnimationClip>,
    pub current: String,
    pub timer: Timer,
    pub frame: usize,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            animations: std::collections::HashMap::new(),
            current: "idle".to_string(),
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame: 0,
        }
    }
    
    pub fn add_animation(&mut self, name: &str, clip: AnimationClip) {
        self.animations.insert(name.to_string(), clip);
    }
    
    pub fn play(&mut self, name: &str) {
        if self.animations.contains_key(name) && self.current != name {
            self.current = name.to_string();
            self.frame = self.animations[name].start;
            self.timer = Timer::from_seconds(
                self.animations[name].frame_time,
                if self.animations[name].looping {
                    TimerMode::Repeating
                } else {
                    TimerMode::Once
                },
            );
        }
    }
    
    pub fn is_finished(&self) -> bool {
        if let Some(clip) = self.animations.get(&self.current) {
            !clip.looping && self.frame >= clip.end
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct AnimationClip {
    pub start: usize,
    pub end: usize,
    pub frame_time: f32,
    pub looping: bool,
}

impl AnimationClip {
    pub fn new(start: usize, end: usize, frame_time: f32, looping: bool) -> Self {
        Self { start, end, frame_time, looping }
    }
}

fn animate_sprites(
    mut query: Query<(&mut AnimationController, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut controller, mut sprite) in query.iter_mut() {
        controller.timer.tick(time.delta());
        
        if controller.timer.just_finished() {
            if let Some(clip) = controller.animations.get(&controller.current) {
                controller.frame += 1;
                if controller.frame > clip.end {
                    if clip.looping {
                        controller.frame = clip.start;
                    } else {
                        controller.frame = clip.end;
                    }
                }
                
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = controller.frame;
                }
            }
        }
    }
