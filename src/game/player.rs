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
