// src/entities/player.rs
use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};
use crate::tilemap::tilemap::{Tile, TileType};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, player_movement);
    }
}

const PLAYER_FRAME_W: u32 = 32;
const PLAYER_FRAME_H: u32 = 32;
const PLAYER_COLUMNS: u32 = 4;
const PLAYER_ROWS: u32 = 4;

#[derive(Component)]
pub struct Player { 
    pub speed: f32, 
    pub health: i32,
    pub size: Vec2,  // Added for collision detection
}

impl Default for Player { 
    fn default() -> Self { 
        Self { 
            speed: 500.0, 
            health: 100,
            size: Vec2::new(24.0, 24.0),  // Slightly smaller than tile for better collision
        } 
    } 
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image: Handle<Image> = asset_server.load("sprites/test_p_sprite.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_FRAME_W, PLAYER_FRAME_H),
        PLAYER_COLUMNS,
        PLAYER_ROWS,
        None, None,
    );
    let layout_handle = layouts.add(layout);

    let mut animation = SpriteSheetAnimation::new(0.1);
    animation.add_animation("idle".into(),   AnimationClip { start_index: 0,  end_index: 3,  frame_duration: 0.2 });
    animation.add_animation("walk".into(),   AnimationClip { start_index: 4,  end_index: 7,  frame_duration: 0.1 });
    animation.add_animation("attack".into(), AnimationClip { start_index: 8,  end_index: 11, frame_duration: 0.05 });
    animation.play("idle", true);

    commands.spawn((
        Player::default(),
        Sprite {
            image,
            texture_atlas: Some(TextureAtlas { layout: layout_handle, index: 0 }),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        animation,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player, &mut SpriteSheetAnimation)>,
    tile_query: Query<(&Transform, &Tile), Without<Player>>,
    time: Res<Time>,
) {
    for (mut player_transform, player, mut animation) in player_query.iter_mut() {
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
            
            // Calculate new position
            let movement = direction * player.speed * time.delta_secs();
            let new_position = player_transform.translation + movement;
            
            // Check collision with walls
            let mut can_move = true;
            for (tile_transform, tile) in tile_query.iter() {
                // Only check collision with walls and other non-walkable tiles
                if !tile.walkable {
                    if check_collision(
                        new_position.truncate(),
                        player.size,
                        tile_transform.translation.truncate(),
                        Vec2::new(32.0, 32.0), // Tile size
                    ) {
                        can_move = false;
                        break;
                    }
                }
            }
            
            // Apply movement if no collision
            if can_move {
                player_transform.translation = new_position;
            } else {
                // Try to slide along walls (move only in non-blocked direction)
                let mut slide_x = player_transform.translation;
                slide_x.x += movement.x;
                let mut slide_y = player_transform.translation;
                slide_y.y += movement.y;
                
                let mut can_move_x = true;
                let mut can_move_y = true;
                
                for (tile_transform, tile) in tile_query.iter() {
                    if !tile.walkable {
                        if check_collision(
                            slide_x.truncate(),
                            player.size,
                            tile_transform.translation.truncate(),
                            Vec2::new(32.0, 32.0),
                        ) {
                            can_move_x = false;
                        }
                        if check_collision(
                            slide_y.truncate(),
                            player.size,
                            tile_transform.translation.truncate(),
                            Vec2::new(32.0, 32.0),
                        ) {
                            can_move_y = false;
                        }
                    }
                }
                
                if can_move_x {
                    player_transform.translation.x = slide_x.x;
                }
                if can_move_y {
                    player_transform.translation.y = slide_y.y;
                }
            }
        } else if animation.current_animation != "idle" {
            animation.play("idle", true);
        }
    }
}

// AABB collision detection
fn check_collision(
    pos1: Vec2,
    size1: Vec2,
    pos2: Vec2,
    size2: Vec2,
) -> bool {
    let half_size1 = size1 / 2.0;
    let half_size2 = size2 / 2.0;
    
    let min1 = pos1 - half_size1;
    let max1 = pos1 + half_size1;
    let min2 = pos2 - half_size2;
    let max2 = pos2 + half_size2;
    
    // Check if boxes overlap
    !(max1.x < min2.x || min1.x > max2.x || max1.y < min2.y || min1.y > max2.y)
}
