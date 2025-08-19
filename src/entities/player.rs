use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (wait_for_map_size, ApplyDeferred, spawn_player).chain())
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
    pub health: i32 
}

impl Default for Player { 
    fn default() -> Self { 
        Self { speed: 250.0, health: 100 } 
    } 
}

fn wait_for_map_size(map_size: Option<Res<crate::tilemap::tilemap::MapSizePx>>) {
    if map_size.is_none() {
        panic!("MapSizePx resource not found!");
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
    animation.add_animation("idle".into(), AnimationClip { start_index: 0, end_index: 3, frame_duration: 0.2 });
    animation.add_animation("walk".into(), AnimationClip { start_index: 4, end_index: 7, frame_duration: 0.1 });
    animation.add_animation("attack".into(), AnimationClip { start_index: 8, end_index: 11, frame_duration: 0.05 });
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
    time: Res<Time>,
    map_size: Option<Res<crate::tilemap::tilemap::MapSizePx>>,
) {
    let Some(map_size) = map_size else { return; };
    
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
            let new_pos = transform.translation + direction * player.speed * time.delta_secs();
            
            // Clamp to map bounds
            let half_w = map_size.w * 0.5 - 16.0;
            let half_h = map_size.h * 0.5 - 16.0;
            transform.translation.x = new_pos.x.clamp(-half_w, half_w);
            transform.translation.y = new_pos.y.clamp(-half_h, half_h);
        } else if animation.current_animation != "idle" {
            animation.play("idle", true);
        }
    }
}
