use bevy::prelude::*;
use crate::systems::talents::{PlayerTalents, TalentRegistry};
use crate::core::state::GameState;

pub struct TalentMenuPlugin;

impl Plugin for TalentMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, talent_menu_input)
            .add_systems(OnEnter(GameState::Paused), setup_talent_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_talent_menu);
    }
}

#[derive(Component)]
pub struct TalentMenu;

#[derive(Component)]
pub struct TalentButton {
    pub talent_id: String,
}

#[derive(Component)]
pub struct TalentPointsDisplay;

fn talent_menu_input(
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        match current_state.get() {
            GameState::Playing => game_state.set(GameState::Paused),
            GameState::Paused => game_state.set(GameState::Playing),
            _ => {}
        }
    }
}

fn setup_talent_menu(
    mut commands: Commands,
    talents: Res<PlayerTalents>,
    talent_registry: Res<TalentRegistry>,
) {
    // Talent UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(5.0),
                top: Val::Percent(5.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            TalentMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Talent Tree"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
            ));
            
            // Available points display
            parent.spawn((
                Text::new(format!("Available Points: {}", talents.available_points)),
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TalentPointsDisplay,
            ));
            
            // Talent trees
            for (tree_id, tree) in talent_registry.trees.iter() {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                    ))
                    .with_children(|tree_parent| {
                        tree_parent.spawn((
                            Text::new(&tree.name),
                            TextColor(Color::srgb(0.0, 1.0, 1.0)),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                        ));
                        
                        // Talents in this tree
                        tree_parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..default()
                                },
                            ))
                            .with_children(|talents_parent| {
                                for (talent_id, talent) in tree.talents.iter() {
                                    let current_level = talents.unlocked_talents
                                        .get(talent_id)
                                        .copied()
                                        .unwrap_or(0);
                                    
                                    let button_color = if current_level > 0 {
                                        Color::srgb(0.0, 0.5, 0.0) // Green if unlocked
                                    } else if talents.available_points > 0 {
                                        Color::srgb(0.2, 0.2, 0.5) // Blue if can afford
                                    } else {
                                        Color::srgb(0.2, 0.2, 0.2) // Gray if can't afford
                                    };
                                    
                                    talents_parent
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(150.0),
                                                height: Val::Px(100.0),
                                                margin: UiRect::all(Val::Px(5.0)),
                                                padding: UiRect::all(Val::Px(10.0)),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            BackgroundColor(button_color),
                                            TalentButton {
                                                talent_id: talent_id.clone(),
                                            },
                                        ))
                                        .with_children(|button| {
                                            button.spawn((
                                                Text::new(&talent.name),
                                                TextColor(Color::WHITE),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                            ));
                                            button.spawn((
                                                Text::new(format!("Level: {}/{}", current_level, talent.max_ranks)),
                                                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                                TextFont {
                                                    font_size: 12.0,
                                                    ..default()
                                                },
                                            ));
                                        });
                                }
                            });
                    });
            }
            
            // Instructions
            parent.spawn((
                Text::new("Press T to close talent menu"),
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
            ));
        });
}

fn cleanup_talent_menu(
    mut commands: Commands,
    talent_menu_q: Query<Entity, With<TalentMenu>>,
) {
    for entity in talent_menu_q.iter() {
        commands.entity(entity).despawn();
    }
}
