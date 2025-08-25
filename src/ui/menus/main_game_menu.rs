use bevy::prelude::*;
use crate::core::state::GameState;

pub struct MainGameMenuPlugin;

impl Plugin for MainGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuState>()
            .add_systems(Update, toggle_menu)
            .add_systems(OnEnter(GameState::Paused), setup_main_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_main_menu)
            .add_systems(Update, (
                handle_tab_switching,
                update_tab_content,
            ).run_if(in_state(GameState::Paused)));
    }
}

#[derive(Resource, Default)]
struct MenuState {
    current_tab: MenuTab,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum MenuTab {
    #[default]
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
struct TabContent;

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

fn setup_main_menu(mut commands: Commands, mut menu_state: ResMut<MenuState>) {
    menu_state.current_tab = MenuTab::Shop;
    
    // Main container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(90.0),
            height: Val::Percent(85.0),
            left: Val::Percent(5.0),
            top: Val::Percent(7.5),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.95)),
        MainMenuUI,
    )).with_children(|parent| {
        // Tab bar
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(60.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        }).with_children(|tab_parent| {
            let tabs = [
                (MenuTab::Shop, "🛒 Shop"),
                (MenuTab::Talents, "⭐ Talents"),
                (MenuTab::Achievements, "🏆 Achievements"),
                (MenuTab::Quests, "📋 Quests"),
                (MenuTab::Inventory, "🎒 Inventory"),
                (MenuTab::Prestige, "♾️ Prestige"),
                (MenuTab::Settings, "⚙️ Settings"),
            ];
            
            for (tab, label) in tabs {
                tab_parent.spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(if tab == MenuTab::Shop {
                        Color::srgb(0.2, 0.3, 0.5)
                    } else {
                        Color::srgb(0.1, 0.1, 0.2)
                    }),
                    TabButton(tab),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(label),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });
        
        // Content area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(20.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            TabContent,
        ));
    });
}

fn handle_tab_switching(
    mut menu_state: ResMut<MenuState>,
    interaction_query: Query<(&TabButton, &Interaction), Changed<Interaction>>,
    mut button_query: Query<(&TabButton, &mut BackgroundColor)>,
) {
    // Check for pressed buttons first
    let mut new_tab = None;
    for (tab_button, interaction) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            new_tab = Some(tab_button.0);
            break;
        }
    }
    
    // If a tab was pressed, update the state and colors
    if let Some(tab) = new_tab {
        menu_state.current_tab = tab;
        
        // Update all button colors
        for (button, mut color) in button_query.iter_mut() {
            *color = if button.0 == menu_state.current_tab {
                BackgroundColor(Color::srgb(0.2, 0.3, 0.5))
            } else {
                BackgroundColor(Color::srgb(0.1, 0.1, 0.2))
            };
        }
    }
}

fn update_tab_content(
    menu_state: Res<MenuState>,
    mut commands: Commands,
    content_query: Query<Entity, With<TabContent>>,
) {
    if !menu_state.is_changed() { return; }
    
    if let Ok(content_entity) = content_query.single() {
        // Clear current content
        commands.entity(content_entity).despawn();
        
        // Spawn new content based on current tab
        commands.entity(content_entity).with_children(|parent| {
            match menu_state.current_tab {
                MenuTab::Shop => spawn_shop_content(parent),
                MenuTab::Talents => spawn_talents_content(parent),
                MenuTab::Achievements => spawn_achievements_content(parent),
                MenuTab::Quests => spawn_quests_content(parent),
                MenuTab::Inventory => spawn_inventory_content(parent),
                MenuTab::Prestige => spawn_prestige_content(parent),
                MenuTab::Settings => spawn_settings_content(parent),
            }
        });
    }
}

fn spawn_shop_content(parent: &mut bevy::ecs::system::EntityCommands) {
    parent.spawn((
        Text::new("Shop"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::srgb(1.0, 0.843, 0.0)),
    ));
    
    // Shop grid will be populated here
}

fn spawn_talents_content(parent: &mut impl bevy::hierarchy::BuildChildren) {
    parent.spawn((
        Text::new("Talent Trees"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::srgb(0.5, 1.0, 0.5)),
    ));
    
    // Talent tree visualization will be here
}

fn spawn_achievements_content(parent: &mut impl bevy::hierarchy::BuildChildren) {
    parent.spawn((
        Text::new("Achievements"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::srgb(1.0, 0.5, 0.0)),
    ));
    
    // Achievement grid will be here
}

fn spawn_quests_content(parent: &mut impl bevy::hierarchy::BuildChildren) {
    parent.spawn((
        Text::new("Quests & Challenges"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::srgb(0.7, 0.7, 1.0)),
    ));
    
    // Quest list will be here
}

fn spawn_inventory_content(parent: &mut impl bevy::hierarchy::BuildChildren) {
    parent.spawn((
        Text::new("Inventory & Loot"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::srgb(0.8, 0.4, 0.8)),
    ));
    
    // Inventory grid will be here
}

fn spawn_prestige_content(parent: &mut impl bevy::hierarchy::BuildChildren) {
    parent.spawn((
        Text::new("Prestige & Meta-Progression"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::srgb(1.0, 0.0, 0.5)),
    ));
    
    // Prestige options will be here
}

fn spawn_settings_content(parent: &mut impl bevy::hierarchy::BuildChildren) {
    parent.spawn((
        Text::new("Settings"),
        TextFont { font_size: 32.0, ..default() },
        TextColor(Color::WHITE),
    ));
}

fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
