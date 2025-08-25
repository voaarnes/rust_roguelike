use bevy::prelude::*;
use crate::core::state::GameState;

pub struct MainGameMenuPlugin;

impl Plugin for MainGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuState>()
            .add_systems(Update, toggle_menu.run_if(not(in_state(GameState::MainMenu))))
            .add_systems(OnEnter(GameState::Paused), setup_main_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_main_menu)
            .add_systems(Update, (
                handle_tab_buttons,
                handle_close_button,
                update_tab_content,
            ).run_if(in_state(GameState::Paused)));
    }
}

#[derive(Resource)]
pub struct MenuState {
    pub current_tab: MenuTab,
    pub previous_tab: Option<MenuTab>,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            current_tab: MenuTab::Shop,
            previous_tab: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuTab {
    Shop,
    Talents,
    Achievements,
    Quests,
    Inventory,
    Prestige,
    Settings,
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct TabButton(MenuTab);

#[derive(Component)]
struct TabContentContainer;

#[derive(Component)]
struct CloseButton;

fn toggle_menu(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::Tab) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

fn setup_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<MenuState>,
) {
    // Reset to shop tab when opening
    menu_state.current_tab = MenuTab::Shop;
    menu_state.previous_tab = None;
    
    // Full screen dark overlay
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        MainMenuUI,
    )).with_children(|overlay| {
        // Main menu container
        overlay.spawn(Node {
            width: Val::Percent(90.0),
            height: Val::Percent(85.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert((
            BorderColor(Color::srgb(0.3, 0.3, 0.4)),
            Outline {
                width: Val::Px(2.0),
                offset: Val::Px(2.0),
                color: Color::srgb(0.1, 0.1, 0.15),
            },
        ))
        .with_children(|menu| {
            // Header with title and close button
            menu.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            }).with_children(|header| {
                // Title
                header.spawn((
                    Text::new("GAME MENU"),
                    TextFont { 
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                ));
                
                // Close button (X)
                header.spawn((
                    Button,
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.1, 0.1)),
                    CloseButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("✕"),
                        TextFont { 
                            font_size: 24.0,
                            ..default() 
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
            
            // Tab buttons
            menu.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            })
            .insert(BackgroundColor(Color::srgb(0.08, 0.08, 0.12)))
            .with_children(|tab_bar| {
                let tabs = [
                    (MenuTab::Shop, "🛒 Shop", Color::srgb(1.0, 0.843, 0.0)),
                    (MenuTab::Talents, "⭐ Talents", Color::srgb(0.5, 1.0, 0.5)),
                    (MenuTab::Achievements, "🏆 Achievements", Color::srgb(1.0, 0.5, 0.0)),
                    (MenuTab::Quests, "📋 Quests", Color::srgb(0.7, 0.7, 1.0)),
                    (MenuTab::Inventory, "🎒 Inventory", Color::srgb(0.8, 0.4, 0.8)),
                    (MenuTab::Prestige, "♾️ Prestige", Color::srgb(1.0, 0.0, 0.5)),
                    (MenuTab::Settings, "⚙️ Settings", Color::srgb(0.6, 0.6, 0.6)),
                ];
                
                for (tab, label, accent_color) in tabs {
                    let is_active = tab == MenuTab::Shop;
                    tab_bar.spawn((
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(15.0), Val::Px(10.0)),
                            margin: UiRect::horizontal(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(if is_active {
                            Color::srgb(0.15, 0.2, 0.3)
                        } else {
                            Color::srgb(0.08, 0.08, 0.12)
                        }),
                        TabButton(tab),
                    )).with_children(|button| {
                        button.spawn((
                            Text::new(label),
                            TextFont { 
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(if is_active { accent_color } else { Color::srgb(0.7, 0.7, 0.7) }),
                        ));
                    });
                }
            });
            
            // Content area with scrolling
            menu.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(20.0)),
                    overflow: Overflow {
                        x: OverflowAxis::Hidden,
                        y: OverflowAxis::Scroll,
                    },
                    ..default()
                },
                BackgroundColor(Color::srgb(0.02, 0.02, 0.05)),
                TabContentContainer,
            )).with_children(|content| {
                // Initial content - Shop tab
                content.spawn((
                    Text::new("🛒 SHOP"),
                    TextFont { 
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.843, 0.0)),
                ));
            });
        });
    });
}

fn handle_tab_buttons(
    mut menu_state: ResMut<MenuState>,
    mut interaction_query: Query<
        (&TabButton, &Interaction, &mut BackgroundColor, &Children),
        Changed<Interaction>
    >,
    mut text_query: Query<&mut TextColor>,
) {
    for (tab_button, interaction, mut bg_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if menu_state.current_tab != tab_button.0 {
                    menu_state.previous_tab = Some(menu_state.current_tab);
                    menu_state.current_tab = tab_button.0;
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.12, 0.15, 0.2));
                for child in children {
                    if let Ok(mut text_color) = text_query.get_mut(*child) {
                        text_color.0 = Color::srgb(1.0, 1.0, 1.0);
                    }
                }
            }
            Interaction::None => {
                let is_active = menu_state.current_tab == tab_button.0;
                *bg_color = BackgroundColor(if is_active {
                    Color::srgb(0.15, 0.2, 0.3)
                } else {
                    Color::srgb(0.08, 0.08, 0.12)
                });
            }
        }
    }
}

fn handle_close_button(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

fn update_tab_content(
    menu_state: Res<MenuState>,
    mut commands: Commands,
    content_query: Query<Entity, With<TabContentContainer>>,
    all_button_query: Query<(&TabButton, Entity)>,
    mut button_bg_query: Query<&mut BackgroundColor>,
    mut text_color_query: Query<&mut TextColor>,
    children_query: Query<&Children>,
) {
    if !menu_state.is_changed() {
        return;
    }
    
    // Update button appearances
    for (tab_button, entity) in &all_button_query {
        let is_active = tab_button.0 == menu_state.current_tab;
        
        if let Ok(mut bg_color) = button_bg_query.get_mut(entity) {
            *bg_color = BackgroundColor(if is_active {
                Color::srgb(0.15, 0.2, 0.3)
            } else {
                Color::srgb(0.08, 0.08, 0.12)
            });
        }
        
        // Update text color of button children
        if let Ok(children) = children_query.get(entity) {
            for child in children {
                if let Ok(mut text_color) = text_color_query.get_mut(*child) {
                    text_color.0 = if is_active {
                        get_tab_color(tab_button.0)
                    } else {
                        Color::srgb(0.7, 0.7, 0.7)
                    };
                }
            }
        }
    }
    
    // Update content
    if let Ok(container) = content_query.single() {
        // Clear existing content - query for children and despawn them
        if let Ok(children) = children_query.get(container) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }
        
        // Spawn new content
        commands.entity(container).with_children(|parent| {
            match menu_state.current_tab {
                MenuTab::Shop => {
                    // Title
                    parent.spawn((
                        Text::new("🛒 SHOP"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                    ));
                    
                    // Shop categories
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(30.0),
                        ..default()
                    }).with_children(|categories| {
                        // Add shop content inline
                        categories.spawn((
                            Text::new("Shop coming soon!"),
                            TextFont { 
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    });
                },
                MenuTab::Talents => {
                    // Inline talent content
                    parent.spawn((
                        Text::new("🌟 TALENTS"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 1.0, 0.5)),
                    ));
                },
                MenuTab::Achievements => {
                    // Inline achievements content
                    parent.spawn((
                        Text::new("🏆 ACHIEVEMENTS"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.5, 0.0)),
                    ));
                },
                MenuTab::Quests => {
                    // Inline quests content
                    parent.spawn((
                        Text::new("📋 QUESTS"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 1.0)),
                    ));
                },
                MenuTab::Inventory => {
                    // Inline inventory content
                    parent.spawn((
                        Text::new("🎒 INVENTORY"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.4, 0.8)),
                    ));
                },
                MenuTab::Prestige => {
                    // Inline prestige content
                    parent.spawn((
                        Text::new("💎 PRESTIGE"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.0, 0.5)),
                    ));
                },
                MenuTab::Settings => {
                    parent.spawn((
                        Text::new("⚙️ Settings"),
                        TextFont { 
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                },
            }
        });
    }
}

fn get_tab_color(tab: MenuTab) -> Color {
    match tab {
        MenuTab::Shop => Color::srgb(1.0, 0.843, 0.0),
        MenuTab::Talents => Color::srgb(0.5, 1.0, 0.5),
        MenuTab::Achievements => Color::srgb(1.0, 0.5, 0.0),
        MenuTab::Quests => Color::srgb(0.7, 0.7, 1.0),
        MenuTab::Inventory => Color::srgb(0.8, 0.4, 0.8),
        MenuTab::Prestige => Color::srgb(1.0, 0.0, 0.5),
        MenuTab::Settings => Color::srgb(0.6, 0.6, 0.6),
    }
}

fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}
