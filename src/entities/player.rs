// src/entities/player.rs
use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 500.0,
            health: 100,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/player_sheet.png");
    
    // Create texture atlas layout for player sprite sheet
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4,  // columns
        4,  // rows
        None,
        None,
    );
    
    let mut animation = SpriteSheetAnimation::new(0.1);
    
    // Define animations
    animation.add_animation(
        "idle".to_string(),
        AnimationClip {
            start_index: 0,
            end_index: 3,
            frame_duration: 0.2,
        },
    );
    
    animation.add_animation(
        "walk".to_string(),
        AnimationClip {
            start_index: 4,
            end_index: 7,
            frame_duration: 0.1,
        },
    );
    
    animation.add_animation(
        "attack".to_string(),
        AnimationClip {
            start_index: 8,
            end_index: 11,
            frame_duration: 0.05,
        },
    );
    
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: Handle::weak_from_u128(0), // You'll need to properly handle this
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player::default(),
        animation,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player, &mut SpriteSheetAnimation)>,
    time: Res<Time>,
) {
    for (mut transform, player, mut animation) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut is_moving = false;

        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            is_moving = true;
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            is_moving = true;
        }
        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            is_moving = true;
        }
        if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            is_moving = true;
        }

        if is_moving {
            if animation.current_animation != "walk" {
                animation.play("walk", true);
            }
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            transform.translation += direction * player.speed * time.delta_secs();
        } else if animation.current_animation != "idle" {
            animation.play("idle", true);
        }
    }
}
