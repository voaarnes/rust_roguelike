use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_movement, enemy_ai));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub health: i32,
    pub direction: Vec2,
    pub detection_range: f32,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    Goblin,
    Skeleton,
    Orc,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Goblin => Self {
                enemy_type,
                speed: 150.0,
                health: 30,
                direction: Vec2::new(1.0, 0.0),
                detection_range: 200.0,
            },
            EnemyType::Skeleton => Self {
                enemy_type,
                speed: 100.0,
                health: 50,
                direction: Vec2::new(0.0, 1.0),
                detection_range: 250.0,
            },
            EnemyType::Orc => Self {
                enemy_type,
                speed: 80.0,
                health: 100,
                direction: Vec2::new(-1.0, 0.0),
                detection_range: 150.0,
            },
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    enemy_type: EnemyType,
) {
    let texture_handle: Handle<Image> = match enemy_type {
        EnemyType::Goblin => asset_server.load("sprites/goblin_sheet.png"),
        EnemyType::Skeleton => asset_server.load("sprites/skeleton_sheet.png"),
        EnemyType::Orc => asset_server.load("sprites/orc_sheet.png"),
    };
    
    let mut animation = SpriteSheetAnimation::new(0.15);
    
    animation.add_animation(
        "idle".to_string(),
        AnimationClip {
            start_index: 0,
            end_index: 3,
            frame_duration: 0.3,
        },
    );
    
    animation.add_animation(
        "walk".to_string(),
        AnimationClip {
            start_index: 4,
            end_index: 7,
            frame_duration: 0.15,
        },
    );
    
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: Handle::weak_from_u128(0),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(position),
        Enemy::new(enemy_type),
        animation,
    ));
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_secs();
    }
}

fn enemy_ai(
    mut enemy_query: Query<(&Transform, &mut Enemy, &mut SpriteSheetAnimation), Without<crate::entities::player::Player>>,
    player_query: Query<&Transform, With<crate::entities::player::Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut enemy, mut animation) in enemy_query.iter_mut() {
            let distance = enemy_transform.translation.distance(player_transform.translation);
            
            if distance < enemy.detection_range {
                // Chase player
                let direction = (player_transform.translation - enemy_transform.translation).truncate().normalize();
                enemy.direction = direction;
                
                if animation.current_animation != "walk" {
                    animation.play("walk", true);
                }
            } else if animation.current_animation != "idle" {
                animation.play("idle", true);
            }
        }
    }
}
