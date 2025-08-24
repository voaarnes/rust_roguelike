use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};
use crate::game::abilities::{ActiveAbilities, AbilityRegistry, BodyPart};
use crate::game::player::Player;
use std::f32::consts::PI;

pub struct PowerUpDisplayPlugin;

impl Plugin for PowerUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FruitAssets>()
            .add_systems(Startup, (load_and_setup_powerup_display,))
            .add_systems(Update, (
                update_powerup_display.run_if(in_state(crate::core::state::GameState::Playing)),
                update_cooldown_timers.run_if(in_state(crate::core::state::GameState::Playing)),
            ));
    }
}

#[derive(Component)]
pub struct PowerUpSlotUI {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct CooldownCircle {
    pub slot_index: usize,
    pub body_part: BodyPart,
}

#[derive(Component)]
pub struct PowerUpContainer;

#[derive(Component)]
pub struct FruitDisplay {
    pub slot_index: usize,
}

#[derive(Resource, Default)]
pub struct FruitAssets {
    pub fruit_atlas: Handle<TextureAtlasLayout>,
    pub fruit_texture: Handle<Image>,
    pub loaded: bool,
}

fn load_and_setup_powerup_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut fruit_assets: ResMut<FruitAssets>,
) {
    // Load fruit sprites
    let texture = asset_server.load("sprites/meyveler.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        8, 1,  // 8 frames in meyveler.png
        None, None,
    );
    let layout_handle = layouts.add(layout);

    fruit_assets.fruit_texture = texture;
    fruit_assets.fruit_atlas = layout_handle;
    fruit_assets.loaded = true;

    // Setup UI
    setup_powerup_display_internal(&mut commands, &fruit_assets);
}

fn setup_powerup_display_internal(
    commands: &mut Commands,
    fruit_assets: &FruitAssets,
) {
    // Main container positioned at bottom right
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            PowerUpContainer,
        ))
        .with_children(|parent| {
            // Create 3 power-up slots (head, torso, legs)
            for i in 0..3 {
                let (label, body_part) = match i {
                    0 => ("HEAD", BodyPart::Head),
                    1 => ("TORSO", BodyPart::Torso),
                    2 => ("LEGS", BodyPart::Legs),
                    _ => ("", BodyPart::Head),
                };
                
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                )).with_children(|label_parent| {
                    // Label
                    label_parent.spawn((
                        Text::new(label),
                        TextFont { font_size: 10.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    
                    // Slot container
                    label_parent.spawn((
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                        BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.9)),
                        PowerUpSlotUI { slot_index: i },
                    )).with_children(|slot_parent| {
                        // Fruit sprite display
                        slot_parent.spawn((
                            Node {
                                width: Val::Px(40.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            ImageNode {
                                image: fruit_assets.fruit_texture.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: fruit_assets.fruit_atlas.clone(),
                                    index: 0, // Default to strawberry, will be updated
                                }),
                                ..default()
                            },
                            Visibility::Hidden, // Hidden until fruit is assigned
                            FruitDisplay { slot_index: i },
                        ));
                        
                        // Cooldown circle overlay
                        slot_parent.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(60.0),
                                height: Val::Px(60.0),
                                border: UiRect::all(Val::Px(2.0)),
                                top: Val::Px(-1.0),
                                left: Val::Px(-1.0),
                                ..default()
                            },
                            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Dark overlay
                            CooldownCircle { 
                                slot_index: i,
                                body_part,
                            },
                        ));
                    });
                });
            }
        });
}

fn update_powerup_display(
    player_query: Query<&PowerUpSlots, With<Player>>,
    mut fruit_query: Query<(&FruitDisplay, &mut ImageNode, &mut Visibility)>,
) {
    if let Ok(powerup_slots) = player_query.single() {
        let fruit_types = powerup_slots.get_fruit_types_as_vec();
        
        for (fruit_display, mut image_node, mut visibility) in fruit_query.iter_mut() {
            if fruit_display.slot_index < fruit_types.len() {
                if let Some(fruit_type) = fruit_types[fruit_display.slot_index] {
                    // Show fruit sprite using actual fruit type (0-7)
                    if let Some(ref mut atlas) = image_node.texture_atlas {
                        atlas.index = fruit_type as usize;
                    }
                    *visibility = Visibility::Visible;
                } else {
                    // Hide sprite when no fruit
                    *visibility = Visibility::Hidden;
                }
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn update_cooldown_timers(
    player_query: Query<&ActiveAbilities, With<Player>>,
    registry: Res<AbilityRegistry>,
    cooldown_query: Query<(Entity, &CooldownCircle)>,
    mut bg_query: Query<&mut BackgroundColor>,
    mut border_query: Query<&mut BorderColor>,
) {
    let Ok(abilities) = player_query.single() else { return };
    
    for (entity, cooldown_circle) in cooldown_query.iter() {
        let ability = match cooldown_circle.body_part {
            BodyPart::Head => &abilities.head_ability,
            BodyPart::Torso => &abilities.torso_ability,
            BodyPart::Legs => &abilities.legs_ability,
        };
        
        if let Some(ability_instance) = ability {
            if let Some(ability_def) = registry.abilities.get(&ability_instance.ability_id) {
                let cooldown_progress = ability_instance.cooldown_timer.elapsed_secs() / ability_def.cooldown;
                let cooldown_progress = cooldown_progress.clamp(0.0, 1.0);
                
                // Update overlay darkness based on cooldown (clock-style)
                if let Ok(mut bg_color) = bg_query.get_mut(entity) {
                    if cooldown_progress >= 1.0 {
                        // Ready - no overlay
                        *bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
                    } else {
                        // On cooldown - dark overlay that gradually reveals the fruit
                        // Create a pulsing effect to simulate the clock sweep
                        let darkness = (1.0 - cooldown_progress) * 0.7;
                        let pulse = (cooldown_progress * PI * 4.0).sin() * 0.1 + 0.9;
                        *bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, darkness * pulse));
                    }
                }
                
                // Update border color with progress indication
                if let Ok(mut border_color) = border_query.get_mut(entity) {
                    if cooldown_progress >= 1.0 {
                        *border_color = BorderColor(Color::srgb(0.0, 1.0, 0.0)); // Green when ready
                    } else {
                        // Orange to yellow transition based on progress
                        let r = 1.0;
                        let g = 0.5 + (cooldown_progress * 0.5);
                        let b = 0.0;
                        *border_color = BorderColor(Color::srgb(r, g, b));
                    }
                }
            }
        } else {
            // No ability assigned
            if let Ok(mut bg_color) = bg_query.get_mut(entity) {
                *bg_color = BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5));
            }
            if let Ok(mut border_color) = border_query.get_mut(entity) {
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}
