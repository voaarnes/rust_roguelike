use bevy::prelude::*;
use crate::core::state::GameState;
use crate::systems::shop::{PlayerCurrency, ShopInventory};
use crate::systems::achievements::{AchievementRegistry, PlayerAchievements};
use crate::systems::talents::{TalentTree, PlayerTalents};
use crate::systems::quests::{QuestManager, ActiveQuests};
use crate::systems::prestige::{PrestigeSystem, MetaProgression};
use crate::systems::loot::CollectedLoot;
use crate::systems::combo::ComboTracker;

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
                update_tab_content.run_if(in_state(GameState::Paused)))
            .add_systems(Update, 
                handle_shop_purchases.run_if(in_state(GameState::Paused)));
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

#[derive(Component)]
struct ShopItemButton {
    item_id: String,
}

#[derive(Component)]
struct CurrencyDisplay;

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
                
                // Shop categories
                content.spawn(Node {
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(30.0),
                    ..default()
                }).with_children(|categories| {
                    // Shop placeholder content
                    categories.spawn((
                        Text::new("⚔️ Weapons - Coming Soon!"),
                        TextFont { 
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.3, 0.3)),
                    ));
                    
                    categories.spawn((
                        Text::new("⬆️ Upgrades - In Development"),
                        TextFont { 
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.3, 0.8, 1.0)),
                    ));
                    
                    categories.spawn((
                        Text::new("📦 Items - Features Planned"),
                        TextFont { 
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.5, 1.0)),
                    ));
                });
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
    // Game system resources - simplified for now
    currency: Res<PlayerCurrency>,
    shop_inventory: Res<ShopInventory>,
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
                    // Title with currency display
                    parent.spawn((
                        Text::new("🛒 SHOP"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                    ));
                    
                    // Currency display
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(30.0),
                            margin: UiRect::vertical(Val::Px(15.0)),
                            padding: UiRect::all(Val::Px(15.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
                        CurrencyDisplay,
                    )).with_children(|currencies| {
                        currencies.spawn((
                            Text::new(format!("💰 Coins: {}", currency.coins)),
                            TextFont { 
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.843, 0.0)),
                        ));
                        currencies.spawn((
                            Text::new(format!("� Gems: {}", currency.gems)),
                            TextFont { 
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 1.0)),
                        ));
                        currencies.spawn((
                            Text::new(format!("👻 Soul Shards: {}", currency.soul_shards)),
                            TextFont { 
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.0, 0.8)),
                        ));
                    });
                    
                    // Shop items grid
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: Val::Px(15.0),
                        row_gap: Val::Px(15.0),
                        ..default()
                    }).with_children(|grid| {
                        // Display actual shop items
                        for item in &shop_inventory.items {
                            let can_afford = match item.currency_type {
                                crate::systems::shop::CurrencyType::Coins => currency.coins >= item.cost,
                                crate::systems::shop::CurrencyType::Gems => currency.gems >= item.cost,
                                crate::systems::shop::CurrencyType::SoulShards => currency.soul_shards >= item.cost,
                            };
                            
                            grid.spawn((
                                Button,
                                Node {
                                    width: Val::Px(200.0),
                                    height: Val::Px(160.0),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(12.0)),
                                    row_gap: Val::Px(8.0),
                                    ..default()
                                },
                                BackgroundColor(if can_afford {
                                    match item.tier {
                                        crate::systems::shop::ItemTier::Common => Color::srgb(0.15, 0.15, 0.2),
                                        crate::systems::shop::ItemTier::Uncommon => Color::srgb(0.1, 0.2, 0.1),
                                        crate::systems::shop::ItemTier::Rare => Color::srgb(0.1, 0.15, 0.25),
                                        crate::systems::shop::ItemTier::Epic => Color::srgb(0.2, 0.1, 0.25),
                                        crate::systems::shop::ItemTier::Legendary => Color::srgb(0.25, 0.2, 0.05),
                                    }
                                } else {
                                    Color::srgb(0.1, 0.1, 0.1)
                                }),
                                ShopItemButton { item_id: item.id.clone() },
                            )).with_children(|item_card| {
                                // Item name
                                item_card.spawn((
                                    Text::new(&item.name),
                                    TextFont { 
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(if can_afford { Color::WHITE } else { Color::srgb(0.5, 0.5, 0.5) }),
                                ));
                                
                                // Item cost
                                let currency_icon = match item.currency_type {
                                    crate::systems::shop::CurrencyType::Coins => "💰",
                                    crate::systems::shop::CurrencyType::Gems => "💎",
                                    crate::systems::shop::CurrencyType::SoulShards => "👻",
                                };
                                item_card.spawn((
                                    Text::new(format!("{} {}", currency_icon, item.cost)),
                                    TextFont { 
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(if can_afford { 
                                        Color::srgb(1.0, 0.843, 0.0) 
                                    } else { 
                                        Color::srgb(0.8, 0.2, 0.2) 
                                    }),
                                ));
                                
                                // Item description
                                item_card.spawn((
                                    Text::new(&item.description),
                                    TextFont { 
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                                
                                // Stock info
                                if item.stock > 0 {
                                    item_card.spawn((
                                        Text::new(format!("Stock: {}", item.stock)),
                                        TextFont { 
                                            font_size: 11.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                    ));
                                }
                            });
                        }
                        
                        // If no items available
                        if shop_inventory.items.is_empty() {
                            grid.spawn((
                                Text::new("🔄 No items available - shop refreshes on wave completion!"),
                                TextFont { 
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            ));
                        }
                    });
                },
                MenuTab::Talents => {
                    // Title and points available
                    parent.spawn((
                        Text::new("⭐ TALENT TREES"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 1.0, 0.5)),
                    ));
                    
                    parent.spawn((
                        Text::new(format!("Available Talent Points: {}", player_talents.available_points)),
                        TextFont { 
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                    ));
                    
                    // Talent trees
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    }).with_children(|trees_container| {
                        // Offense Tree
                        if let Some(offense_tree) = talent_tree.trees.get(&crate::systems::talents::TalentTreeType::Offense) {
                            trees_container.spawn(Node {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(15.0)),
                                row_gap: Val::Px(10.0),
                                ..default()
                            }).insert(BackgroundColor(Color::srgb(0.15, 0.05, 0.05))).with_children(|tree| {
                                tree.spawn((
                                    Text::new("⚔️ Offense Tree"),
                                    TextFont { 
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.3, 0.3)),
                                ));
                                
                                let points_spent = player_talents.spent_points.get(&crate::systems::talents::TalentTreeType::Offense).copied().unwrap_or(0);
                                tree.spawn((
                                    Text::new(format!("Points Spent: {}", points_spent)),
                                    TextFont { 
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                
                                // Show some talents
                                for (_id, talent) in offense_tree.talents.iter().take(3) {
                                    let current_rank = player_talents.unlocked_talents.get(&talent.id).copied().unwrap_or(0);
                                    tree.spawn(Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::all(Val::Px(8.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    }).insert(BackgroundColor(if current_rank > 0 {
                                        Color::srgb(0.1, 0.3, 0.1)
                                    } else {
                                        Color::srgb(0.1, 0.1, 0.15)
                                    })).with_children(|talent_node| {
                                        talent_node.spawn((
                                            Text::new(format!("{} ({}/{})", talent.name, current_rank, talent.max_ranks)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                }
                            });
                        }
                        
                        // Defense Tree
                        if let Some(defense_tree) = talent_tree.trees.get(&crate::systems::talents::TalentTreeType::Defense) {
                            trees_container.spawn(Node {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(15.0)),
                                row_gap: Val::Px(10.0),
                                ..default()
                            }).insert(BackgroundColor(Color::srgb(0.05, 0.05, 0.15))).with_children(|tree| {
                                tree.spawn((
                                    Text::new("🛡️ Defense Tree"),
                                    TextFont { 
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.3, 0.5, 1.0)),
                                ));
                                
                                let points_spent = player_talents.spent_points.get(&crate::systems::talents::TalentTreeType::Defense).copied().unwrap_or(0);
                                tree.spawn((
                                    Text::new(format!("Points Spent: {}", points_spent)),
                                    TextFont { 
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                
                                // Show some talents
                                for (_id, talent) in defense_tree.talents.iter().take(3) {
                                    let current_rank = player_talents.unlocked_talents.get(&talent.id).copied().unwrap_or(0);
                                    tree.spawn(Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::all(Val::Px(8.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    }).insert(BackgroundColor(if current_rank > 0 {
                                        Color::srgb(0.1, 0.3, 0.1)
                                    } else {
                                        Color::srgb(0.1, 0.1, 0.15)
                                    })).with_children(|talent_node| {
                                        talent_node.spawn((
                                            Text::new(format!("{} ({}/{})", talent.name, current_rank, talent.max_ranks)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                }
                            });
                        }
                        
                        // Utility Tree  
                        if let Some(utility_tree) = talent_tree.trees.get(&crate::systems::talents::TalentTreeType::Utility) {
                            trees_container.spawn(Node {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(15.0)),
                                row_gap: Val::Px(10.0),
                                ..default()
                            }).insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.05))).with_children(|tree| {
                                tree.spawn((
                                    Text::new("✨ Utility Tree"),
                                    TextFont { 
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.8, 0.3)),
                                ));
                                
                                let points_spent = player_talents.spent_points.get(&crate::systems::talents::TalentTreeType::Utility).copied().unwrap_or(0);
                                tree.spawn((
                                    Text::new(format!("Points Spent: {}", points_spent)),
                                    TextFont { 
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));
                                
                                // Show some talents
                                for (_id, talent) in utility_tree.talents.iter().take(3) {
                                    let current_rank = player_talents.unlocked_talents.get(&talent.id).copied().unwrap_or(0);
                                    tree.spawn(Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::all(Val::Px(8.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    }).insert(BackgroundColor(if current_rank > 0 {
                                        Color::srgb(0.1, 0.3, 0.1)
                                    } else {
                                        Color::srgb(0.1, 0.1, 0.15)
                                    })).with_children(|talent_node| {
                                        talent_node.spawn((
                                            Text::new(format!("{} ({}/{})", talent.name, current_rank, talent.max_ranks)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                }
                            });
                        }
                    });
                },
                MenuTab::Achievements => {
                    // Title
                    parent.spawn((
                        Text::new("🏆 ACHIEVEMENTS"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.5, 0.0)),
                    ));
                    
                    // Achievement stats
                    let total_achievements = achievement_registry.achievements.len();
                    let unlocked_count = player_achievements.unlocked.values().filter(|&&v| v).count();
                    
                    parent.spawn((
                        Text::new(format!("Progress: {}/{} Unlocked ({:.1}%)", 
                            unlocked_count, 
                            total_achievements,
                            if total_achievements > 0 { (unlocked_count as f32 / total_achievements as f32) * 100.0 } else { 0.0 }
                        )),
                        TextFont { 
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                    ));
                    
                    // Achievement grid
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(15.0),
                        column_gap: Val::Px(15.0),
                        ..default()
                    }).with_children(|grid| {
                        // Group achievements by category
                        use std::collections::HashMap;
                        let mut categorized: HashMap<crate::systems::achievements::AchievementCategory, Vec<_>> = HashMap::new();
                        
                        for (id, achievement) in &achievement_registry.achievements {
                            categorized.entry(achievement.category).or_default().push((id, achievement));
                        }
                        
                        for (category, achievements) in categorized {
                            // Category header
                            grid.spawn(Node {
                                width: Val::Percent(100.0),
                                margin: UiRect::vertical(Val::Px(10.0)),
                                ..default()
                            }).with_children(|cat_header| {
                                cat_header.spawn((
                                    Text::new(format!("{:?} Achievements", category)),
                                    TextFont { 
                                        font_size: 22.0,
                                        ..default()
                                    },
                                    TextColor(match category {
                                        crate::systems::achievements::AchievementCategory::Combat => Color::srgb(1.0, 0.3, 0.3),
                                        crate::systems::achievements::AchievementCategory::Collection => Color::srgb(0.3, 1.0, 0.3),
                                        crate::systems::achievements::AchievementCategory::Progression => Color::srgb(0.3, 0.3, 1.0),
                                        crate::systems::achievements::AchievementCategory::Exploration => Color::srgb(1.0, 0.8, 0.3),
                                        crate::systems::achievements::AchievementCategory::Challenge => Color::srgb(1.0, 0.3, 1.0),
                                        crate::systems::achievements::AchievementCategory::Secret => Color::srgb(0.8, 0.8, 0.8),
                                    }),
                                ));
                            });
                            
                            // Achievement cards
                            for (id, achievement) in achievements {
                                let is_unlocked = player_achievements.unlocked.get(id).copied().unwrap_or(false);
                                let progress = player_achievements.progress.get(id).copied().unwrap_or(0);
                                
                                grid.spawn(Node {
                                    width: Val::Px(180.0),
                                    height: Val::Px(120.0),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    row_gap: Val::Px(5.0),
                                    ..default()
                                }).insert(BackgroundColor(if is_unlocked {
                                    match achievement.tier {
                                        crate::systems::achievements::AchievementTier::Bronze => Color::srgb(0.4, 0.25, 0.1),
                                        crate::systems::achievements::AchievementTier::Silver => Color::srgb(0.3, 0.3, 0.3),
                                        crate::systems::achievements::AchievementTier::Gold => Color::srgb(0.5, 0.4, 0.1),
                                        crate::systems::achievements::AchievementTier::Platinum => Color::srgb(0.2, 0.4, 0.5),
                                        crate::systems::achievements::AchievementTier::Diamond => Color::srgb(0.3, 0.1, 0.5),
                                    }
                                } else {
                                    Color::srgb(0.1, 0.1, 0.1)
                                })).with_children(|card| {
                                    // Achievement name
                                    card.spawn((
                                        Text::new(&achievement.name),
                                        TextFont { 
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(if is_unlocked { Color::WHITE } else { Color::srgb(0.5, 0.5, 0.5) }),
                                    ));
                                    
                                    // Description
                                    card.spawn((
                                        Text::new(&achievement.description),
                                        TextFont { 
                                            font_size: 11.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                    
                                    // Tier and progress
                                    card.spawn((
                                        Text::new(if is_unlocked {
                                            format!("✅ {:?}", achievement.tier)
                                        } else {
                                            format!("Progress: {}", progress)
                                        }),
                                        TextFont { 
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(if is_unlocked { 
                                            Color::srgb(0.5, 1.0, 0.5) 
                                        } else { 
                                            Color::srgb(0.8, 0.8, 0.3) 
                                        }),
                                    ));
                                });
                            }
                        }
                        
                        // If no achievements
                        if achievement_registry.achievements.is_empty() {
                            grid.spawn((
                                Text::new("🔄 Achievement system initializing..."),
                                TextFont { 
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            ));
                        }
                    });
                },
                MenuTab::Quests => {
                    // Title
                    parent.spawn((
                        Text::new("📋 QUESTS & CHALLENGES"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 1.0)),
                    ));
                    
                    // Active quests section
                    parent.spawn((
                        Text::new(format!("Active Quests ({}/{})", 
                            active_quests.daily_quests.len() + 
                            active_quests.wave_challenges.len() + 
                            active_quests.story_quests.len(),
                            quest_manager.available_quests.len()
                        )),
                        TextFont { 
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                    ));
                    
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            margin: UiRect::vertical(Val::Px(20.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                    )).with_children(|quest_list| {
                        // Show active quests
                        let all_active_quests = [
                            &active_quests.daily_quests[..],
                            &active_quests.wave_challenges[..],
                            &active_quests.story_quests[..],
                        ].concat();
                        
                        for active_quest in &all_active_quests {
                            let quest = &active_quest.quest;
                            quest_list.spawn(Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(15.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.0),
                                    ..default()
                                }).insert(BackgroundColor(match quest.quest_type {
                                    crate::systems::quests::QuestType::Daily => Color::srgb(0.1, 0.15, 0.1),
                                    crate::systems::quests::QuestType::Wave => Color::srgb(0.15, 0.1, 0.1),
                                    crate::systems::quests::QuestType::Story => Color::srgb(0.1, 0.1, 0.15),
                                    crate::systems::quests::QuestType::Hidden => Color::srgb(0.1, 0.1, 0.1),
                                    crate::systems::quests::QuestType::Challenge => Color::srgb(0.15, 0.1, 0.15),
                                })).with_children(|quest_card| {
                                    // Quest name and type
                                    quest_card.spawn(Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    }).with_children(|header| {
                                        header.spawn((
                                            Text::new(&quest.name),
                                            TextFont { 
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                        
                                        header.spawn((
                                            Text::new(format!("{:?}", quest.quest_type)),
                                            TextFont { 
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(match quest.quest_type {
                                                crate::systems::quests::QuestType::Daily => Color::srgb(0.5, 1.0, 0.5),
                                                crate::systems::quests::QuestType::Wave => Color::srgb(1.0, 0.5, 0.5),
                                                crate::systems::quests::QuestType::Story => Color::srgb(0.5, 0.5, 1.0),
                                                crate::systems::quests::QuestType::Hidden => Color::srgb(0.8, 0.8, 0.8),
                                                crate::systems::quests::QuestType::Challenge => Color::srgb(1.0, 0.5, 1.0),
                                            }),
                                        ));
                                    });
                                    
                                    // Quest description
                                    quest_card.spawn((
                                        Text::new(&quest.description),
                                        TextFont { 
                                            font_size: 13.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    ));
                                    
                                    // Progress bar
                                    quest_card.spawn(Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(20.0),
                                        padding: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    }).insert(BackgroundColor(Color::srgb(0.2, 0.2, 0.2))).with_children(|progress_container| {
                                        // Calculate progress from objectives
                                        let total_objectives = quest.objectives.len() as f32;
                                        let completed_objectives = active_quest.progress.len() as f32;
                                        let progress_percent = if total_objectives > 0.0 {
                                            (completed_objectives / total_objectives).min(1.0)
                                        } else {
                                            0.0
                                        };
                                        
                                        progress_container.spawn(Node {
                                            width: Val::Percent(progress_percent * 100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        }).insert(BackgroundColor(if progress_percent >= 1.0 {
                                            Color::srgb(0.2, 0.8, 0.2)
                                        } else {
                                            Color::srgb(0.8, 0.6, 0.2)
                                        }));
                                    });
                                    
                                    // Progress text and rewards
                                    quest_card.spawn(Node {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    }).with_children(|footer| {
                                        footer.spawn((
                                            Text::new(format!("Progress: {}/{}", 
                                                active_quest.progress.len(), 
                                                quest.objectives.len()
                                            )),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                        ));
                                        
                                        footer.spawn((
                                            Text::new(format!("Reward: {} XP", quest.rewards.experience)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(1.0, 0.843, 0.0)),
                                        ));
                                    });
                                });
                            }
                        }
                        
                        // If no active quests
                        if all_active_quests.is_empty() {
                            quest_list.spawn((
                                Text::new("📭 No active quests - new quests appear as you progress!"),
                                TextFont { 
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            ));
                        }
                        
                        // Available quests section
                        if !quest_manager.available_quests.is_empty() {
                            quest_list.spawn((
                                Text::new("Available Quests"),
                                TextFont { 
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 1.0)),
                            ));
                            
                            for (id, quest) in quest_manager.available_quests.iter().take(3) {
                                if !all_active_quests.iter().any(|aq| &aq.quest.id == id) {
                                    quest_list.spawn(Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    }).insert(BackgroundColor(Color::srgb(0.05, 0.05, 0.1))).with_children(|available_quest| {
                                        available_quest.spawn((
                                            Text::new(&quest.name),
                                            TextFont { 
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));
                                        
                                        available_quest.spawn((
                                            Text::new(format!("{:?}", quest.quest_type)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                        ));
                                    });
                                }
                            }
                        }
                    });
                },
                MenuTab::Inventory => {
                    // Title
                    parent.spawn((
                        Text::new("🎒 INVENTORY & LOOT"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.4, 0.8)),
                    ));
                    
                    // Loot statistics
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(15.0)),
                        padding: UiRect::all(Val::Px(15.0)),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: Val::Px(20.0),
                        row_gap: Val::Px(10.0),
                        ..default()
                    }).insert(BackgroundColor(Color::srgb(0.05, 0.05, 0.08))).with_children(|stats| {
                        stats.spawn((
                            Text::new("Loot Statistics:"),
                            TextFont { 
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.843, 0.0)),
                        ));
                        
                        for (rarity, count) in &collected_loot.total_items {
                            let color = match rarity {
                                crate::systems::loot::Rarity::Common => Color::srgb(0.8, 0.8, 0.8),
                                crate::systems::loot::Rarity::Uncommon => Color::srgb(0.3, 1.0, 0.3),
                                crate::systems::loot::Rarity::Rare => Color::srgb(0.3, 0.5, 1.0),
                                crate::systems::loot::Rarity::Epic => Color::srgb(0.8, 0.3, 1.0),
                                crate::systems::loot::Rarity::Legendary => Color::srgb(1.0, 0.5, 0.0),
                            };
                            
                            stats.spawn((
                                Text::new(format!("{:?}: {}", rarity, count)),
                                TextFont { 
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(color),
                            ));
                        }
                    });
                    
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(30.0),
                        ..default()
                    }).with_children(|main_container| {
                        // Equipment section (left side)
                        main_container.spawn(Node {
                            width: Val::Percent(40.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        }).with_children(|equipment| {
                            equipment.spawn((
                                Text::new("⚔️ Equipment"),
                                TextFont { 
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.843, 0.0)),
                            ));
                            
                            // Show equipment slots
                            for equipment_item in &collected_loot.equipment {
                                equipment.spawn(Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(10.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(5.0),
                                    ..default()
                                }).insert(BackgroundColor(Color::srgb(0.1, 0.15, 0.2))).with_children(|item| {
                                    item.spawn((
                                        Text::new(&equipment_item.name),
                                        TextFont { 
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                    
                                    item.spawn((
                                        Text::new(format!("{:?}", equipment_item.slot)),
                                        TextFont { 
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));
                                    
                                    // Show some stats
                                    for (stat, value) in equipment_item.stats.iter().take(2) {
                                        item.spawn((
                                            Text::new(format!("+{} {:?}", value, stat)),
                                            TextFont { 
                                                font_size: 11.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 1.0, 0.5)),
                                        ));
                                    }
                                });
                            }
                            
                            if collected_loot.equipment.is_empty() {
                                equipment.spawn((
                                    Text::new("No equipment found yet"),
                                    TextFont { 
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                ));
                            }
                        });
                        
                        // Consumables section (right side)
                        main_container.spawn(Node {
                            width: Val::Percent(60.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        }).with_children(|consumables| {
                            consumables.spawn((
                                Text::new("📦 Consumables & Materials"),
                                TextFont { 
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.843, 0.0)),
                            ));
                            
                            // Consumables grid
                            consumables.spawn(Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                column_gap: Val::Px(10.0),
                                row_gap: Val::Px(10.0),
                                ..default()
                            }).with_children(|grid| {
                                // Show materials (using materials instead of consumables)
                                for (material, count) in &collected_loot.materials {
                                    grid.spawn(Node {
                                        width: Val::Px(80.0),
                                        height: Val::Px(80.0),
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(8.0)),
                                        ..default()
                                    }).insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.15))).with_children(|item| {
                                        item.spawn((
                                            Text::new(format!("{:?}", material)),
                                            TextFont { 
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                        
                                        item.spawn((
                                            Text::new(format!("x{}", count)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(1.0, 0.843, 0.0)),
                                        ));
                                    });
                                }
                                
                                // Show materials
                                for (material, count) in &collected_loot.materials {
                                    grid.spawn(Node {
                                        width: Val::Px(80.0),
                                        height: Val::Px(80.0),
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(8.0)),
                                        ..default()
                                    }).insert(BackgroundColor(Color::srgb(0.15, 0.1, 0.1))).with_children(|item| {
                                        item.spawn((
                                            Text::new(format!("{:?}", material)),
                                            TextFont { 
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                        
                                        item.spawn((
                                            Text::new(format!("x{}", count)),
                                            TextFont { 
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(1.0, 0.843, 0.0)),
                                        ));
                                    });
                                }
                                
                                // Empty slots to fill grid  
                                for _ in 0..(20 - collected_loot.materials.len() - collected_loot.equipment.len()).min(20) {
                                    grid.spawn(Node {
                                        width: Val::Px(80.0),
                                        height: Val::Px(80.0),
                                        ..default()
                                    }).insert(BackgroundColor(Color::srgb(0.05, 0.05, 0.05)));
                                }
                            });
                        });
                    });
                },
                MenuTab::Prestige => {
                    // Title
                    parent.spawn((
                        Text::new("💎 PRESTIGE SYSTEM"),
                        TextFont { 
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.0, 0.5)),
                    ));
                    
                    // Current prestige info
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    }).insert(BackgroundColor(Color::srgb(0.1, 0.05, 0.15))).with_children(|info| {
                        info.spawn((
                            Text::new(format!("Current Prestige Level: {}", prestige_system.current_prestige)),
                            TextFont { 
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.843, 0.0)),
                        ));
                        
                        info.spawn((
                            Text::new(format!("Total Prestiges: {}", prestige_system.total_prestiges)),
                            TextFont { 
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                        
                        info.spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(30.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        }).with_children(|currencies| {
                            currencies.spawn((
                                Text::new(format!("💎 Prestige Points: {}", prestige_system.prestige_points)),
                                TextFont { 
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 1.0, 0.5)),
                            ));
                            
                            currencies.spawn((
                                Text::new(format!("⭐ Legacy Points: {}", prestige_system.legacy_points)),
                                TextFont { 
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.5, 0.0)),
                            ));
                            
                            currencies.spawn((
                                Text::new(format!("✨ Ascension Shards: {}", prestige_system.ascension_shards)),
                                TextFont { 
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.5, 1.0)),
                            ));
                        });
                    });
                    
                    // Meta upgrades section
                    parent.spawn((
                        Text::new("🌟 Permanent Upgrades"),
                        TextFont { 
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.5, 0.0)),
                    ));
                    
                    parent.spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(15.0),
                        column_gap: Val::Px(15.0),
                        margin: UiRect::vertical(Val::Px(15.0)),
                        ..default()
                    }).with_children(|upgrades_grid| {
                        // Show meta upgrades
                        for (id, upgrade) in &meta_progression.permanent_upgrades {
                            upgrades_grid.spawn(Node {
                                width: Val::Px(200.0),
                                height: Val::Px(120.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(12.0)),
                                row_gap: Val::Px(6.0),
                                ..default()
                            }).insert(BackgroundColor(Color::srgb(0.15, 0.15, 0.2))).with_children(|upgrade_card| {
                                upgrade_card.spawn((
                                    Text::new(&upgrade.name),
                                    TextFont { 
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                                
                                upgrade_card.spawn((
                                    Text::new(&upgrade.description),
                                    TextFont { 
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                                
                                upgrade_card.spawn((
                                    Text::new(format!("Level: {}/{}", upgrade.current_level, upgrade.max_level)),
                                    TextFont { 
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.843, 0.0)),
                                ));
                                
                                // Show if maxed
                                if upgrade.current_level >= upgrade.max_level {
                                    upgrade_card.spawn((
                                        Text::new("✅ MAXED"),
                                        TextFont { 
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.5, 1.0, 0.5)),
                                    ));
                                }
                            });
                        }
                        
                        // If no upgrades
                        if meta_progression.permanent_upgrades.is_empty() {
                            upgrades_grid.spawn((
                                Text::new("Complete your first prestige to unlock permanent upgrades!"),
                                TextFont { 
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                            ));
                        }
                    });
                    
                    // Unlocked features
                    if !meta_progression.unlocked_features.is_empty() {
                        parent.spawn((
                            Text::new("🔓 Unlocked Features"),
                            TextFont { 
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 1.0, 0.5)),
                        ));
                        
                        parent.spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            column_gap: Val::Px(15.0),
                            row_gap: Val::Px(10.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        }).with_children(|features| {
                            for feature in &meta_progression.unlocked_features {
                                features.spawn(Node {
                                    padding: UiRect::all(Val::Px(8.0)),
                                    ..default()
                                }).insert(BackgroundColor(Color::srgb(0.1, 0.2, 0.1))).with_children(|feature_badge| {
                                    feature_badge.spawn((
                                        Text::new(feature),
                                        TextFont { 
                                            font_size: 12.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.8, 1.0, 0.8)),
                                    ));
                                });
                            }
                        });
                    }
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
                    
                    parent.spawn((
                        Text::new("\nSettings menu coming soon!\n\n• Audio controls\n• Graphics options\n• Key bindings\n• Gameplay preferences"),
                        TextFont { 
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
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
        commands.entity(entity).despawn();
    }
}

fn handle_shop_purchases(
    mut interaction_query: Query<(&Interaction, &ShopItemButton), Changed<Interaction>>,
    mut purchase_events: EventWriter<crate::systems::shop::PurchaseEvent>,
    player_query: Query<Entity, With<crate::game::player::Player>>,
) {
    for (interaction, shop_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(player_entity) = player_query.single() {
                purchase_events.write(crate::systems::shop::PurchaseEvent {
                    item_id: shop_button.item_id.clone(),
                    player: player_entity,
                });
            }
        }
    }
}
