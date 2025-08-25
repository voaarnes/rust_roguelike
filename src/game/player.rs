use bevy::prelude::*;
use crate::game::animation::{AnimationController, AnimationClip};
use crate::entities::powerup::PowerUpSlots;
use crate::game::movement::{Velocity, Collider};
use crate::game::combat::{Health, CombatStats};
use crate::systems::talents::PlayerTalents;
use crate::game::player_visual::PlayerParts;
use crate::game::abilities::ActiveAbilities;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerResources>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_input_system,
                update_player_stats,
                apply_speed_buffs,
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
pub struct PlayerStats {
    pub kills: u32,
    pub coins_collected: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            kills: 0,
            coins_collected: 0,
            damage_dealt: 0,
            damage_taken: 0,
        }
    }
}

#[derive(Component)]
pub struct PlayerController {
    pub move_speed: f32,
    pub dash_speed: f32,
    pub dash_cooldown: Timer,
    pub is_dashing: bool,
}

#[derive(Resource, Default)]
pub struct PlayerResources {
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
        PlayerStats::default(),
        PlayerController::default(),
        PowerUpSlots::new(3),
        ActiveAbilities::default(),
        Health::new(100),
        CombatStats {
            damage: 10,
            armor: 5,
            crit_chance: 0.1,
            crit_multiplier: 2.0,
        },
        Velocity(Vec2::ZERO),
        Collider { size: Vec2::splat(28.0) },
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
        PlayerParts::default(),
    ));
}

/// System to handle player input and movement
fn player_input_system(
    mut player_q: Query<(&mut Velocity, &mut AnimationController, &PlayerController), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    _time: Res<Time>,
) {
    let Ok((mut velocity, mut anim, controller)) = player_q.single_mut() else { return };
    
    let mut movement = Vec2::ZERO;
    
    // Handle WASD movement
    if keys.pressed(KeyCode::KeyW) { movement.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { movement.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { movement.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { movement.x += 1.0; }
    
    // Normalize diagonal movement
    if movement.length() > 0.0 {
        movement = movement.normalize();
        velocity.0 = movement * controller.move_speed;
        
        // Play walk animation
        if anim.current != "walk" {
            anim.play("walk");
        }
    } else {
        velocity.0 = Vec2::ZERO;
        
        // Play idle animation
        if anim.current != "idle" {
            anim.play("idle");
        }
    }
    
    // Handle dash
    if keys.just_pressed(KeyCode::ShiftLeft) && !controller.is_dashing {
        if movement.length() > 0.0 {
            velocity.0 = movement * controller.dash_speed;
            anim.play("dash");
        }
    }
}

fn update_player_stats(
    mut player_q: Query<&mut Player>,
    mut talents: ResMut<PlayerTalents>,
    _time: Res<Time>,
) {
    for mut player in player_q.iter_mut() {
        // Simple level progression
        if player.experience >= player.exp_to_next_level {
            let old_level = player.level;
            player.level += 1;
            player.experience = 0;
            player.exp_to_next_level = player.level * 100;
            
            // Award talent point for leveling up
            let levels_gained = player.level - old_level;
            talents.available_points += levels_gained;
            
            println!("Level up! Now level {}. Gained {} talent points.", player.level, levels_gained);
        }
    }
}

#[derive(Component)]
pub struct SpeedBuff {
    pub multiplier: f32,
    pub duration: Timer,
}

fn apply_speed_buffs(
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut PlayerController, Option<&mut SpeedBuff>), With<Player>>,
    time: Res<Time>,
) {
    for (entity, mut controller, speed_buff) in player_q.iter_mut() {
        if let Some(mut buff) = speed_buff {
            buff.duration.tick(time.delta());
            
            if buff.duration.finished() {
                // Remove expired buff and reset speed
                commands.entity(entity).remove::<SpeedBuff>();
                controller.move_speed = 200.0; // Reset to base speed
            } else {
                // Apply speed multiplier
                controller.move_speed = 200.0 * buff.multiplier;
            }
        } else {
            // No buff active, ensure base speed
            controller.move_speed = 200.0;
        }
    }
}
