/*!
 * Main Game Menu System
 * 
 * This module provides the in-game pause menu with tabbed interface for accessing
 * various game systems including shop, talents, achievements, quests, inventory,
 * prestige, and settings.
 * 
 * Features:
 * - Tabbed navigation with visual feedback
 * - Currency display integration
 * - Placeholder content for future feature implementation
 * - ESC key toggle functionality
 * - Responsive UI design compatible with Bevy 0.16
 */

use bevy::prelude::*;
use crate::core::state::GameState;
use crate::systems::shop::PlayerCurrency;

pub struct MainGameMenuPlugin;

impl Plugin for MainGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuState>()
            .add_systems(Update, toggle_menu.run_if(not(in_state(GameState::MainMenu))))
            .add_systems(OnEnter(GameState::Paused), setup_main_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_main_menu)
            .add_systems(Update, 
                handle_tab_buttons.run_if(in_state(GameState::Paused)))
            .add_systems(Update, 
                handle_close_button.run_if(in_state(GameState::Paused)))
            .add_systems(Update, 
                update_tab_content.run_if(in_state(GameState::Paused)));
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

#[derive(Clone, Copy, PartialEq, Debug)]
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
pub struct MainMenuUI;

#[derive(Component)]
pub struct TabButton(pub MenuTab);

#[derive(Component)]
pub struct TabContentContainer;

#[derive(Component)]
pub struct CloseButton;

/// Handles ESC key input to toggle between Playing and Paused game states
fn toggle_menu(
    input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

/// Creates the main menu UI with tabbed interface when entering Paused state
fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(90.0),
            height: Val::Percent(85.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(5.0),
            top: Val::Percent(7.5),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderColor(Color::srgb(0.3, 0.3, 0.4)),
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.95)),
        MainMenuUI,
    )).with_children(|parent| {
        // Tab bar
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
        )).with_children(|tab_bar| {
            // Define tab configuration with labels and colors
            let tabs = [
                (MenuTab::Shop, "🛒 Shop", Color::srgb(1.0, 0.843, 0.0)),
                (MenuTab::Talents, "⭐ Talents", Color::srgb(0.5, 1.0, 0.5)),
                (MenuTab::Achievements, "🏆 Achievements", Color::srgb(1.0, 0.5, 0.0)),
                (MenuTab::Quests, "📋 Quests", Color::srgb(0.7, 0.7, 1.0)),
                (MenuTab::Inventory, "🎒 Inventory", Color::srgb(0.8, 0.4, 0.8)),
                (MenuTab::Prestige, "♾️ Prestige", Color::srgb(1.0, 0.0, 0.5)),
                (MenuTab::Settings, "⚙️ Settings", Color::srgb(0.6, 0.6, 0.6)),
            ];
            
            // Create tab buttons with active state styling
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
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(if is_active { accent_color } else { Color::srgb(0.7, 0.7, 0.7) }),
                    ));
                });
            }
        });

        // Close button
        parent.spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
            CloseButton,
        )).with_children(|button| {
            button.spawn((
                Text::new("✕"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

        // Content container
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(20.0)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            TabContentContainer,
        ));
    });
}

/// Handles tab button interactions and updates the current menu state
fn handle_tab_buttons(
    mut interaction_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut menu_state: ResMut<MenuState>,
) {
    for (interaction, tab_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            menu_state.previous_tab = Some(menu_state.current_tab);
            menu_state.current_tab = tab_button.0;
        }
    }
}

/// Handles close button interaction to return to Playing state
fn handle_close_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

/// Updates tab content and button styling based on current menu state
/// This system handles both visual feedback for active tabs and content switching
fn update_tab_content(
    menu_state: Res<MenuState>,
    mut commands: Commands,
    content_query: Query<Entity, With<TabContentContainer>>,
    all_button_query: Query<(&TabButton, Entity)>,
    mut button_bg_query: Query<&mut BackgroundColor>,
    mut text_color_query: Query<&mut TextColor>,
    children_query: Query<&Children>,
    currency: Res<PlayerCurrency>,
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

    // Update content based on current tab selection
    if let Ok(container) = content_query.single() {
        // Clear existing content before spawning new
        if let Ok(children) = children_query.get(container) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        // Spawn new content based on current tab
        commands.entity(container).with_children(|parent| {
            match menu_state.current_tab {
                MenuTab::Shop => {
                    // Shop tab - currency display and placeholder content
                    parent.spawn((
                        Text::new("🛒 Shop"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                    ));
                    
                    // Display current currency amounts
                    parent.spawn((
                        Text::new(format!("💰 Coins: {} | 💎 Gems: {} | 🔮 Soul Shards: {}", 
                            currency.coins, currency.gems, currency.soul_shards)),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    
                    // Placeholder content for upcoming shop features
                    parent.spawn((
                        Text::new("\nShop items coming soon!\n\n• Power upgrades\n• Equipment\n• Consumables\n• Special abilities"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
                MenuTab::Talents => {
                    // Talents tab - skill tree placeholder
                    parent.spawn((
                        Text::new("⭐ Talents"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(0.5, 1.0, 0.5)),
                    ));
                    
                    // Placeholder content for talent system
                    parent.spawn((
                        Text::new("\nTalent trees coming soon!\n\n• Offensive specializations\n• Defensive abilities\n• Utility skills\n• Progression paths"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
                MenuTab::Achievements => {
                    // Achievements tab - milestone tracking placeholder
                    parent.spawn((
                        Text::new("🏆 Achievements"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.5, 0.0)),
                    ));
                    
                    // Placeholder content for achievement system
                    parent.spawn((
                        Text::new("\nAchievement system coming soon!\n\n• Combat milestones\n• Collection goals\n• Exploration rewards\n• Challenge completions"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
                MenuTab::Quests => {
                    // Quests tab - mission system placeholder
                    parent.spawn((
                        Text::new("📋 Quests"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 1.0)),
                    ));
                    
                    // Placeholder content for quest system
                    parent.spawn((
                        Text::new("\nQuest system coming soon!\n\n• Daily challenges\n• Wave objectives\n• Story progression\n• Special missions"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
                MenuTab::Inventory => {
                    // Inventory tab - item management placeholder
                    parent.spawn((
                        Text::new("🎒 Inventory"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.4, 0.8)),
                    ));
                    
                    // Placeholder content for inventory system
                    parent.spawn((
                        Text::new("\nInventory system coming soon!\n\n• Equipment management\n• Item storage\n• Loot organization\n• Gear optimization"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
                MenuTab::Prestige => {
                    // Prestige tab - meta progression placeholder
                    parent.spawn((
                        Text::new("♾️ Prestige"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.0, 0.5)),
                    ));
                    
                    // Placeholder content for prestige system
                    parent.spawn((
                        Text::new("\nPrestige system coming soon!\n\n• Meta progression\n• Permanent upgrades\n• Rebirth bonuses\n• Ascension rewards"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
                MenuTab::Settings => {
                    // Settings tab - configuration options placeholder
                    parent.spawn((
                        Text::new("⚙️ Settings"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                    
                    // Placeholder content for settings menu
                    parent.spawn((
                        Text::new("\nSettings menu coming soon!\n\n• Audio controls\n• Graphics options\n• Key bindings\n• Gameplay preferences"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                },
            }
        });
    }
}

/// Cleanup function to remove main menu UI when exiting Paused state
fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
}

/// Helper function to get the accent color for each menu tab
/// Used for consistent theming across the interface
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
