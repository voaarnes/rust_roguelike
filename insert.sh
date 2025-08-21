#!/bin/bash

echo "Applying Bevy 0.16 fixes for macOS..."

# Fix 1: Create missing audio module
cat > src/game/audio/mod.rs << 'EOF'
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, _app: &mut App) {
        // Audio implementation
    }
}
EOF

# Fix 2: Remove powerup_display reference from ui/mod.rs
cat > src/ui/mod.rs << 'EOF'
pub mod main_menu;
pub mod pause_menu;
pub mod hud;
pub mod health_bars;
pub mod minimap;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hud::HUDPlugin,
            main_menu::MainMenuPlugin,
            pause_menu::PauseMenuPlugin,
            health_bars::HealthBarPlugin,
            minimap::MinimapPlugin,
        ));
    }
}
EOF

# Fix 3: Create world/tilemap.rs
cat > src/world/tilemap.rs << 'EOF'
use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, _app: &mut App) {
        // Tilemap implementation
    }
}
EOF

# Fix 4: Fix ui/hud.rs with proper Bevy 0.16 imports
cat > src/ui/hud.rs << 'EOF'
use bevy::prelude::*;
use crate::core::state::GameStats;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameStats>()
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct WaveText;

fn setup_hud(mut commands: Commands) {
    // Score display
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
    
    // Health display  
    commands.spawn((
        TextBundle::from_section(
            "Health: 100/100",
            TextStyle {
                font_size: 24.0,
                color: Color::linear_rgb(0.0, 1.0, 0.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HealthText,
    ));
    
    // Wave display
    commands.spawn((
        TextBundle::from_section(
            "Wave: 1",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        WaveText,
    ));
}

fn update_hud(
    mut score_q: Query<&mut Text, (With<ScoreText>, Without<HealthText>, Without<WaveText>)>,
    mut health_q: Query<&mut Text, (With<HealthText>, Without<ScoreText>, Without<WaveText>)>,
    mut wave_q: Query<&mut Text, (With<WaveText>, Without<ScoreText>, Without<HealthText>)>,
    player_q: Query<&crate::game::combat::Health, With<crate::game::player::Player>>,
    stats: Res<GameStats>,
    wave_manager: Res<crate::game::spawning::WaveManager>,
) {
    for mut text in score_q.iter_mut() {
        *text = Text::from_section(
            format!("Score: {}", stats.score),
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        );
    }
    
    if let Ok(health) = player_q.get_single() {
        for mut text in health_q.iter_mut() {
            let color = if health.percentage() > 0.6 {
                Color::linear_rgb(0.0, 1.0, 0.0)
            } else if health.percentage() > 0.3 {
                Color::linear_rgb(1.0, 1.0, 0.0)
            } else {
                Color::linear_rgb(1.0, 0.0, 0.0)
            };
            
            *text = Text::from_section(
                format!("Health: {}/{}", health.current, health.max),
                TextStyle {
                    font_size: 24.0,
                    color,
                    ..default()
                },
            );
        }
    }
    
    for mut text in wave_q.iter_mut() {
        *text = Text::from_section(
            format!("Wave: {}", wave_manager.current_wave),
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        );
    }
}
EOF

# Fix 5: Fix core/camera.rs
cat > src/core/camera.rs << 'EOF'
use bevy::prelude::*;
use bevy::core_pipeline::core_2d::Camera2dBundle;

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
    mut cam_q: Query<(Entity, &mut Transform, &mut CameraShake)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut shake) in cam_q.iter_mut() {
        shake.duration.tick(time.delta());
        
        if !shake.duration.finished() {
            let progress = shake.duration.fraction_remaining();
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * shake.intensity * progress,
                (rand::random::<f32>() - 0.5) * shake.intensity * progress,
            );
            transform.translation += offset.extend(0.0);
        } else {
            commands.entity(entity).remove::<CameraShake>();
        }
    }
}
EOF

# Fix 6: Fix core/input.rs
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
    let buffer_time = buffer.buffer_time;
    buffer.buffer.retain(|action| {
        current_time - action.timestamp < buffer_time
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
    
    while buffer.buffer.len() > buffer.max_size {
        buffer.buffer.pop_front();
    }
}

pub fn pause_game_system(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<crate::core::state::GameState>>,
    mut next_state: ResMut<NextState<crate::core::state::GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            crate::core::state::GameState::Playing => {
                next_state.set(crate::core::state::GameState::Paused);
            }
            crate::core::state::GameState::Paused => {
                next_state.set(crate::core::state::GameState::Playing);
            }
            _ => {}
        }
    }
}
EOF

# Fix 7: Fix game/combat/damage.rs
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
            if immunity.is_some() {
                continue;
            }
            
            let mut final_damage = event.damage;
            final_damage = (final_damage - stats.armor).max(1);
            
            final_damage = match event.damage_type {
                DamageType::True => event.damage,
                DamageType::Magic => (final_damage as f32 * 1.2) as i32,
                _ => final_damage,
            };
            
            health.take_damage(final_damage);
            spawn_damage_number(&mut commands, event.position, final_damage, event.damage_type);
            
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
    
    // For Bevy 0.16, we use a Sprite as a placeholder
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(30.0, 20.0)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 20.0, 100.0)),
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
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut DamageNumber)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut sprite, mut damage_num) in query.iter_mut() {
        damage_num.lifetime.tick(time.delta());
        
        transform.translation += damage_num.velocity.extend(0.0) * time.delta_secs();
        damage_num.velocity.y -= 200.0 * time.delta_secs();
        
        let alpha = damage_num.lifetime.fraction_remaining();
        sprite.color = sprite.color.with_alpha(alpha);
        
        if damage_num.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
EOF

# Fix 8: Create a proper fix script for deprecated methods
cat > fix_methods.py << 'EOF'
#!/usr/bin/env python3
import os
import re

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Fix deprecated methods
    content = re.sub(r'\.get_single\(\)', '.get_single()', content)
    content = re.sub(r'\.get_single_mut\(\)', '.get_single_mut()', content)
    content = re.sub(r'\.send\(', '.write(', content)
    content = re.sub(r'\.despawn_recursive\(\)', '.despawn()', content)
    content = re.sub(r'time\.delta_seconds\(\)', 'time.delta_secs()', content)
    content = re.sub(r'time\.elapsed_seconds\(\)', 'time.elapsed_secs()', content)
    content = re.sub(r'\.percent_left\(\)', '.fraction_remaining()', content)
    
    with open(filepath, 'w') as f:
        f.write(content)

# Walk through src directory
for root, dirs, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            filepath = os.path.join(root, file)
            fix_file(filepath)
            print(f"Fixed: {filepath}")

print("All files updated!")
EOF

# Make it executable and run
chmod +x fix_methods.py
python3 fix_methods.py

echo "All fixes applied! Now compile with: cargo build"
