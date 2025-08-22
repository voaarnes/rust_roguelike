use bevy::prelude::*;
use crate::game::player::Player;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PlayerVisualPlugin;

impl Plugin for PlayerVisualPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player_parts)
            .add_systems(Update, update_player_appearance);
    }
}

#[derive(Component)]
pub struct PlayerParts {
    pub head_entity: Option<Entity>,
    pub chest_entity: Option<Entity>,
    pub legs_entity: Option<Entity>,
}

#[derive(Component)]
pub struct PlayerPartType {
    pub part_type: PartType,
}

#[derive(Clone, Copy)]
pub enum PartType {
    Head,
    Chest,
    Legs,
}

fn setup_player_parts(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok(player_entity) = player_query.single() else { return };
    
    let texture = asset_server.load("sprites/player_parts.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        8, 6,
        None, None,
    );
    let layout_handle = layouts.add(layout);
    
    // Spawn head (default grape design at index 6)
    let head_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 6, // Grape head as default
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 1.0), // Slightly above center
        PlayerPartType { part_type: PartType::Head },
    )).id();
    
    // Spawn chest (default grape design at index 22)
    let chest_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 22, // Grape chest as default
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0), // Center
        PlayerPartType { part_type: PartType::Chest },
    )).id();
    
    // Spawn legs (default grape design at index 38)
    let legs_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 38, // Grape legs as default
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -8.0, 1.0), // Slightly below center
        PlayerPartType { part_type: PartType::Legs },
    )).id();
    
    // Add PlayerParts component to player
    commands.entity(player_entity).insert(PlayerParts {
        head_entity: Some(head_entity),
        chest_entity: Some(chest_entity),
        legs_entity: Some(legs_entity),
    });
    
    // Make parts children of player
    commands.entity(player_entity).add_children(&[head_entity, chest_entity, legs_entity]);
}

fn update_player_appearance(
    player_query: Query<(&PlayerParts, &PowerUpSlots, &Transform), With<Player>>,
    mut part_query: Query<&mut Sprite>,
) {
    let Ok((player_parts, powerup_slots, _player_transform)) = player_query.single() else { return };
    
    // Get head fruit (newest/first in queue)
    let head_fruit = powerup_slots.get_head_fruit();
    
    // Get legs fruit (oldest/last in queue)
    let legs_fruit = powerup_slots.get_legs_fruit();
    
    // Update head appearance
    if let Some(head_entity) = player_parts.head_entity {
        if let Ok(mut sprite) = part_query.get_mut(head_entity) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = get_head_sprite_index(head_fruit);
            }
        }
    }
    
    // Update chest appearance (use head fruit for now)
    if let Some(chest_entity) = player_parts.chest_entity {
        if let Ok(mut sprite) = part_query.get_mut(chest_entity) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = get_chest_sprite_index(head_fruit);
            }
        }
    }
    
    // Update legs appearance
    if let Some(legs_entity) = player_parts.legs_entity {
        if let Ok(mut sprite) = part_query.get_mut(legs_entity) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = get_legs_sprite_index(legs_fruit);
            }
        }
    }
}

fn get_head_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 1,      // Strawberry head
        Some(PowerUpType::DamageBoost) => 4,     // Apple head
        Some(PowerUpType::HealthBoost) => 5,     // Orange head
        Some(PowerUpType::ShieldBoost) => 7,     // Banana head
        None => 6,                                // Grape head (default)
    }
}

fn get_chest_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 17,     // Strawberry chest
        Some(PowerUpType::DamageBoost) => 20,    // Apple chest
        Some(PowerUpType::HealthBoost) => 21,    // Orange chest
        Some(PowerUpType::ShieldBoost) => 23,    // Banana chest
        None => 22,                               // Grape chest (default)
    }
}

fn get_legs_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 33,     // Strawberry legs
        Some(PowerUpType::DamageBoost) => 36,    // Apple legs
        Some(PowerUpType::HealthBoost) => 37,    // Orange legs
        Some(PowerUpType::ShieldBoost) => 39,    // Banana legs
        None => 38,                               // Grape legs (default)
    }
}
