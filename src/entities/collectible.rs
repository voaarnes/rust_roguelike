use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_collectibles);
    }
}

#[derive(Component)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub value: i32,
}

#[derive(Clone, Copy)]
pub enum CollectibleType {
    Coin,
    Gem,
    HealthPotion,
    ManaPotion,
}

pub fn spawn_collectible(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    collectible_type: CollectibleType,
) {
    let (texture_path, value) = match collectible_type {
        CollectibleType::Coin => ("sprites/coin_sheet.png", 1),
        CollectibleType::Gem => ("sprites/gem_sheet.png", 10),
        CollectibleType::HealthPotion => ("sprites/health_potion_sheet.png", 50),
        CollectibleType::ManaPotion => ("sprites/mana_potion_sheet.png", 30),
    };
    
    let texture_handle: Handle<Image> = asset_server.load(texture_path);
    
    let mut animation = SpriteSheetAnimation::new(0.1);
    animation.add_animation(
        "spin".to_string(),
        AnimationClip {
            start_index: 0,
            end_index: 7,
            frame_duration: 0.1,
        },
    );
    animation.play("spin", true);
    
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
        Collectible {
            collectible_type,
            value,
        },
        animation,
    ));
}

fn animate_collectibles(
    mut query: Query<&mut Transform, With<Collectible>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        // Add a subtle floating animation
        transform.translation.y += (time.elapsed_secs() * 2.0).sin() * 0.5;
    }
}
