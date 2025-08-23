
use bevy::prelude::*;
use crate::game::player::Player;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PlayerVisualPlugin;

impl Plugin for PlayerVisualPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                setup_player_parts.run_if(player_needs_parts),
                update_player_appearance,
            ).chain());
        
        println!("PlayerVisualPlugin initialized!");
    }
}

#[derive(Component, Default, Debug)]
pub struct PlayerParts {
    pub head_entity: Option<Entity>,
    pub chest_entity: Option<Entity>,
    pub legs_entity: Option<Entity>,
    pub initialized: bool,
}

#[derive(Component)]
pub struct PlayerPartType {
    pub part_type: PartType,
}

#[derive(Clone, Copy, Debug)]
pub enum PartType {
    Head,
    Chest,
    Legs,
}

fn player_needs_parts(
    player_query: Query<&PlayerParts, With<Player>>,
) -> bool {
    if let Ok(parts) = player_query.single() {
        !parts.initialized
    } else {
        false
    }
}

fn setup_player_parts(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut PlayerParts, &mut Sprite), With<Player>>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Attempting to setup player parts...");
    
    let Ok((player_entity, mut player_parts, mut player_sprite)) = player_query.single_mut() else {
        return;
    };
    
    if player_parts.initialized {
        return;
    }
    
    println!("Setting up player parts for entity {:?}", player_entity);
    
    let texture = asset_server.load("sprites/player_parts.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        8, 6,
        None, None,
    );
    let layout_handle = layouts.add(layout);
    
    // Spawn head with default grey (index 0)
    let head_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 16.0, 0.1),
        PlayerPartType { part_type: PartType::Head },
    )).id();
    
    // Spawn chest with default grey (index 16)
    let chest_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 16,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.1),
        PlayerPartType { part_type: PartType::Chest },
    )).id();
    
    // Spawn legs with default grey (index 32)
    let legs_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 32,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -16.0, 0.1),
        PlayerPartType { part_type: PartType::Legs },
    )).id();
    
    player_parts.head_entity = Some(head_entity);
    player_parts.chest_entity = Some(chest_entity);
    player_parts.legs_entity = Some(legs_entity);
    player_parts.initialized = true;
    
    commands.entity(player_entity).add_children(&[head_entity, chest_entity, legs_entity]);
    
    // Hide the original sprite
    player_sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
    
    println!("Player parts setup complete!");
}

fn update_player_appearance(
    player_query: Query<(&PlayerParts, &PowerUpSlots), (With<Player>, Changed<PowerUpSlots>)>,
    mut part_query: Query<&mut Sprite>,
) {
    for (player_parts, powerup_slots) in player_query.iter() {
        if !player_parts.initialized {
            continue;
        }
        
        // Get fruits based on their position in the queue
        let head_fruit = powerup_slots.get_head_fruit();    // Newest (index 0)
        let torso_fruit = powerup_slots.get_torso_fruit();  // Middle (index 1)
        let legs_fruit = powerup_slots.get_legs_fruit();     // Oldest (index 2)
        
        println!("Updating appearance - Head: {:?}, Torso: {:?}, Legs: {:?}", 
                 head_fruit, torso_fruit, legs_fruit);
        
        // Update head
        if let Some(head_entity) = player_parts.head_entity {
            if let Ok(mut sprite) = part_query.get_mut(head_entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = get_head_sprite_index(head_fruit);
                }
            }
        }
        
        // Update chest - FIX: Use torso_fruit, not head_fruit!
        if let Some(chest_entity) = player_parts.chest_entity {
            if let Ok(mut sprite) = part_query.get_mut(chest_entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = get_chest_sprite_index(torso_fruit);
                }
            }
        }
        
        // Update legs
        if let Some(legs_entity) = player_parts.legs_entity {
            if let Ok(mut sprite) = part_query.get_mut(legs_entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = get_legs_sprite_index(legs_fruit);
                }
            }
        }
    }
}

// Based on player_parts_guide.txt - Row 0
fn get_head_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 1,      // Strawberry
        Some(PowerUpType::DamageBoost) => 3,     // Mango
        Some(PowerUpType::HealthBoost) => 5,     // Orange
        Some(PowerUpType::ShieldBoost) => 7,     // Banana
        None => 0,                                // Default grey
    }
}

// Based on player_parts_guide.txt - Row 2
fn get_chest_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 17,     // Strawberry
        Some(PowerUpType::DamageBoost) => 19,    // Mango
        Some(PowerUpType::HealthBoost) => 21,    // Orange
        Some(PowerUpType::ShieldBoost) => 23,    // Banana
        None => 16,                               // Default grey
    }
}

// Based on player_parts_guide.txt - Row 4
fn get_legs_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 33,     // Strawberry
        Some(PowerUpType::DamageBoost) => 35,    // Mango
        Some(PowerUpType::HealthBoost) => 37,    // Orange
        Some(PowerUpType::ShieldBoost) => 39,    // Banana
        None => 32,                               // Default grey
    }
}

