use bevy::prelude::*;
use crate::states::GameState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(Update, handle_pause_menu_input.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu);
    }
}

#[derive(Component)]
struct PauseMenuUI;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct MainMenuButton;

fn setup_pause_menu(mut commands: Commands) {
    // Root UI node - semi-transparent overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            PauseMenuUI,
        ))
        .with_children(|parent| {
            // Menu container
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(40.0)),
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // Resume Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            ResumeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("RESUME"),
                                TextFont {
                                    font_size: 35.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });

                    // Main Menu Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            MainMenuButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("MAIN MENU"),
                                TextFont {
                                    font_size: 35.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });
                });
        });
}

fn handle_pause_menu_input(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&ResumeButton>, Option<&MainMenuButton>), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    game_entities: Query<Entity, (Without<Window>, Without<PauseMenuUI>)>,
) {
    for (interaction, mut color, resume, main_menu) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if resume.is_some() {
                    next_state.set(GameState::InGame);
                } else if main_menu.is_some() {
                    // Clean up all game entities when returning to main menu
                    for entity in &game_entities {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(GameState::MainMenu);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
