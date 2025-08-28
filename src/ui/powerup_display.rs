use bevy::prelude::*;
use bevy::ui::*;
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
struct CooldownSegment {
    slot_index: usize,
    segment_index: usize,
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
                        
                        // Create circular cooldown segments container
                        let _cooldown_container = slot_parent.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(60.0),
                                height: Val::Px(60.0),
                                top: Val::Px(-1.0),
                                left: Val::Px(-1.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            CooldownCircle { 
                                slot_index: i,
                                body_part,
                            },
                        )).with_children(|segments_parent| {
                            // Create 12 circular segments for clock-style animation
                            for segment_idx in 0..12 {
                                let angle = (segment_idx as f32) * (PI * 2.0 / 12.0) - PI / 2.0; // Start from top
                                let radius = 25.0; // Distance from center
                                let segment_size = 8.0;
                                
                                // Calculate position for this segment
                                let x = angle.cos() * radius;
                                let y = angle.sin() * radius;
                                
                                segments_parent.spawn((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Px(segment_size),
                                        height: Val::Px(segment_size),
                                        left: Val::Px(30.0 + x - segment_size / 2.0), // Center point + offset
                                        top: Val::Px(30.0 + y - segment_size / 2.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Start dark
                                    CooldownSegment { 
                                        slot_index: i,
                                        segment_index: segment_idx,
                                    },
                                ));
                            }
                        }).id();
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
    circles_query: Query<(&CooldownCircle, Entity)>,
    mut segments_query: Query<(&CooldownSegment, &mut BackgroundColor)>,
    abilities_query: Query<&ActiveAbilities, With<Player>>,
) {
    if let Ok(abilities) = abilities_query.single() {
        for (circle, _circle_entity) in circles_query.iter() {
            let cooldown_percent = match circle.body_part {
                BodyPart::Head => abilities.head_ability.as_ref()
                    .map(|ability| ability.cooldown_timer.elapsed().as_secs_f32() / ability.cooldown_timer.duration().as_secs_f32())
                    .unwrap_or(1.0)
                    .min(1.0),
                BodyPart::Torso => abilities.torso_ability.as_ref()
                    .map(|ability| ability.cooldown_timer.elapsed().as_secs_f32() / ability.cooldown_timer.duration().as_secs_f32())
                    .unwrap_or(1.0)
                    .min(1.0),
                BodyPart::Legs => abilities.legs_ability.as_ref()
                    .map(|ability| ability.cooldown_timer.elapsed().as_secs_f32() / ability.cooldown_timer.duration().as_secs_f32())
                    .unwrap_or(1.0)
                    .min(1.0),
            };
            
            // Calculate how many segments should be bright (clock sweep)
            let segments_to_light = (cooldown_percent * 12.0) as usize;
            
            for (segment, mut bg_color) in segments_query.iter_mut() {
                if segment.slot_index == circle.slot_index {
                    let is_lit = segment.segment_index < segments_to_light;
                    *bg_color = if is_lit {
                        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.8)) // Bright
                    } else {
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)) // Dark
                    };
                }
            }
        }
    }
}