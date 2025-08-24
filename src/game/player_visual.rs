// src/game/player_visual.rs

use bevy::prelude::*;
use crate::game::player::Player;
use crate::entities::powerup::PowerUpSlots;
use crate::world::tilemap::InWater;

/// Plugin that manages player visual appearance based on equipped fruits
pub struct PlayerVisualPlugin;

impl Plugin for PlayerVisualPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                setup_player_parts.run_if(player_needs_parts),
                update_player_appearance,
                handle_water_visual_effects,
            ).chain());
    }
}

/// Component tracking the player's visual parts (head, chest, legs)
#[derive(Component, Default, Debug)]
pub struct PlayerParts {
    pub head_entity: Option<Entity>,
    pub chest_entity: Option<Entity>,
    pub legs_entity: Option<Entity>,
    pub initialized: bool,
}

/// Marker component to identify which body part a sprite represents
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

/// System to set up player visual parts when a player entity is created
fn setup_player_parts(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut PlayerParts, &mut Sprite), With<Player>>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok((player_entity, mut player_parts, mut player_sprite)) = player_query.single_mut() else {
        return;
    };
    
    if player_parts.initialized {
        return;
    }
    
    let texture = asset_server.load("sprites/playe_tmp_sprite.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        7, 3,
        None, None,
    );
    let layout_handle = layouts.add(layout);
    
    // Spawn head with default (first column, row 0)
    let head_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 0, // Default head (column 0, row 0)
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 16.0, 0.1),
        PlayerPartType { part_type: PartType::Head },
    )).id();
    
    // Spawn chest with default (first column, row 1)
    let chest_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 7, // Default chest (column 0, row 1)
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.1),
        PlayerPartType { part_type: PartType::Chest },
    )).id();
    
    // Spawn legs with default (first column, row 2)
    let legs_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 14, // Default legs (column 0, row 2)
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
    
    // Hide the original sprite since we're using parts now
    player_sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
}

/// System to update player appearance based on equipped fruits
fn update_player_appearance(
    player_query: Query<(&PlayerParts, &PowerUpSlots), (With<Player>, Changed<PowerUpSlots>)>,
    mut part_query: Query<&mut Sprite>,
) {
    for (player_parts, powerup_slots) in player_query.iter() {
        if !player_parts.initialized {
            continue;
        }
        
        // Get fruit types for visual updates based on slot position
        let head_fruit = powerup_slots.get_head_fruit();    // Newest (index 0)
        let torso_fruit = powerup_slots.get_torso_fruit();  // Middle (index 1)
        let legs_fruit = powerup_slots.get_legs_fruit();     // Oldest (index 2)
        
        // Update head sprite
        if let Some(head_entity) = player_parts.head_entity {
            if let Ok(mut sprite) = part_query.get_mut(head_entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = get_head_sprite_index(head_fruit);
                }
            }
        }
        
        // Update chest sprite
        if let Some(chest_entity) = player_parts.chest_entity {
            if let Ok(mut sprite) = part_query.get_mut(chest_entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = get_chest_sprite_index(torso_fruit);
                }
            }
        }
        
        // Update legs sprite
        if let Some(legs_entity) = player_parts.legs_entity {
            if let Ok(mut sprite) = part_query.get_mut(legs_entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = get_legs_sprite_index(legs_fruit);
                }
            }
        }
    }
}

/// Map fruit types to head sprite indices (Row 0: indices 0-6)
fn get_head_sprite_index(fruit_type: Option<u8>) -> usize {
    match fruit_type {
        Some(0) => 0,  // Strawberry (column 0, row 0)
        Some(1) => 1,  // Pear (column 1, row 0)
        Some(2) => 2,  // Mango (column 2, row 0)
        Some(3) => 3,  // Pineapple (column 3, row 0)
        Some(4) => 4,  // Apple (column 4, row 0)
        Some(5) => 5,  // Carrot (column 5, row 0)
        Some(6) => 6,  // Coconut (column 6, row 0)
        _ => 0,        // Default (column 0, row 0)
    }
}

/// Map fruit types to chest sprite indices (Row 1: indices 7-13)
fn get_chest_sprite_index(fruit_type: Option<u8>) -> usize {
    match fruit_type {
        Some(0) => 7,  // Strawberry (column 0, row 1)
        Some(1) => 8,  // Pear (column 1, row 1)
        Some(2) => 9,  // Mango (column 2, row 1)
        Some(3) => 10, // Pineapple (column 3, row 1)
        Some(4) => 11, // Apple (column 4, row 1)
        Some(5) => 12, // Carrot (column 5, row 1)
        Some(6) => 13, // Coconut (column 6, row 1)
        _ => 7,        // Default (column 0, row 1)
    }
}

/// Map fruit types to legs sprite indices (Row 2: indices 14-20)
fn get_legs_sprite_index(fruit_type: Option<u8>) -> usize {
    match fruit_type {
        Some(0) => 14, // Strawberry (column 0, row 2)
        Some(1) => 15, // Pear (column 1, row 2)
        Some(2) => 16, // Mango (column 2, row 2)
        Some(3) => 17, // Pineapple (column 3, row 2)
        Some(4) => 18, // Apple (column 4, row 2)
        Some(5) => 19, // Carrot (column 5, row 2)
        Some(6) => 20, // Coconut (column 6, row 2)
        _ => 14,       // Default (column 0, row 2)
    }
}

/// Handle visual effects when player is in water
fn handle_water_visual_effects(
    player_query: Query<(&PlayerParts, Option<&InWater>), With<Player>>,
    mut part_query: Query<(&mut Transform, &PlayerPartType), Without<Player>>,
) {
    if let Ok((parts, water_info)) = player_query.single() {
        if let Some(water) = water_info {
            // When in water, hide body parts based on water depth
            // Only show head when in deep water
            if let Some(head_entity) = parts.head_entity {
                if let Ok((mut transform, part_type)) = part_query.get_mut(head_entity) {
                    match part_type.part_type {
                        PartType::Head => {
                            // Keep head visible always
                            transform.translation.y = 8.0;
                        },
                        _ => {}
                    }
                }
            }
            
            // Hide or show chest based on water depth
            if let Some(chest_entity) = parts.chest_entity {
                if let Ok((mut transform, part_type)) = part_query.get_mut(chest_entity) {
                    match part_type.part_type {
                        PartType::Chest => {
                            if water.depth > 0.5 {
                                // Hide chest when more than half submerged
                                transform.translation.y = -1000.0; // Move off-screen
                            } else {
                                transform.translation.y = 0.0; // Normal position
                            }
                        },
                        _ => {}
                    }
                }
            }
            
            if let Some(legs_entity) = parts.legs_entity {
                if let Ok((mut transform, part_type)) = part_query.get_mut(legs_entity) {
                    match part_type.part_type {
                        PartType::Legs => {
                            if water.depth > 0.2 {
                                // Hide legs when more than 20% submerged
                                transform.translation.y = -1000.0; // Move off-screen
                            } else {
                                transform.translation.y = -8.0; // Normal position
                            }
                        },
                        _ => {}
                    }
                }
            }
        } else {
            // Not in water - reset all positions to normal
            if let Some(head_entity) = parts.head_entity {
                if let Ok((mut transform, part_type)) = part_query.get_mut(head_entity) {
                    if matches!(part_type.part_type, PartType::Head) {
                        transform.translation.y = 8.0;
                    }
                }
            }
            
            if let Some(chest_entity) = parts.chest_entity {
                if let Ok((mut transform, part_type)) = part_query.get_mut(chest_entity) {
                    if matches!(part_type.part_type, PartType::Chest) {
                        transform.translation.y = 0.0;
                    }
                }
            }
            
            if let Some(legs_entity) = parts.legs_entity {
                if let Ok((mut transform, part_type)) = part_query.get_mut(legs_entity) {
                    if matches!(part_type.part_type, PartType::Legs) {
                        transform.translation.y = -8.0;
                    }
                }
            }
        }
    }
}
