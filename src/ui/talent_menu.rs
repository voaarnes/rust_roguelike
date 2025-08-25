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
            NodeBundle {
                style: Style {
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(5.0),
                    top: Val::Percent(5.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            TalentMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Talent Tree",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            // Available points display
            parent.spawn((
                TextBundle::from_section(
                    format!("Available Points: {}", talents.available_points),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                TalentPointsDisplay,
            ));
            
            // Talent trees
            for (tree_id, tree) in talent_registry.trees.iter() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|tree_parent| {
                        tree_parent.spawn(TextBundle::from_section(
                            &tree.name,
                            TextStyle {
                                font_size: 24.0,
                                color: Color::srgb(0.0, 1.0, 1.0),
                                ..default()
                            },
                        ));
                        
                        // Talents in this tree
                        tree_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|talents_parent| {
                                for talent in tree.talents.iter() {
                                    let current_level = talents.unlocked_talents
                                        .get(&talent.id)
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
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(150.0),
                                                    height: Val::Px(100.0),
                                                    margin: UiRect::all(Val::Px(5.0)),
                                                    padding: UiRect::all(Val::Px(10.0)),
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                },
                                                background_color: button_color.into(),
                                                ..default()
                                            },
                                            TalentButton {
                                                talent_id: talent.id.clone(),
                                            },
                                        ))
                                        .with_children(|button| {
                                            button.spawn(TextBundle::from_section(
                                                &talent.name,
                                                TextStyle {
                                                    font_size: 14.0,
                                                    color: Color::WHITE,
                                                    ..default()
                                                },
                                            ));
                                            button.spawn(TextBundle::from_section(
                                                format!("Level: {}/{}", current_level, talent.max_level),
                                                TextStyle {
                                                    font_size: 12.0,
                                                    color: Color::srgb(0.5, 0.5, 0.5),
                                                    ..default()
                                                },
                                            ));
                                        });
                                }
                            });
                    });
            }
            
            // Instructions
            parent.spawn(TextBundle::from_section(
                "Press T to close talent menu",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.5, 0.5, 0.5),
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
        commands.entity(entity).despawn_recursive();
    }
}
