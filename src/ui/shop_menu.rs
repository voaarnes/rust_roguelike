use bevy::prelude::*;
use bevy::ui::{NodeBundle, Style, Val, PositionType, FlexDirection, UiRect, ButtonBundle, TextBundle, TextStyle};
use crate::systems::shop::PlayerCurrency;
use crate::core::state::GameState;

pub struct ShopMenuPlugin;

impl Plugin for ShopMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                shop_menu_input,
                update_shop_display,
            ))
            .add_systems(OnEnter(GameState::Paused), setup_shop_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_shop_menu);
    }
}

#[derive(Component)]
pub struct ShopMenu;

#[derive(Component)]
pub struct ShopItemButton {
    pub item_id: String,
}

#[derive(Component)]
pub struct CurrencyDisplay;

fn shop_menu_input(
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        match current_state.get() {
            GameState::Playing => game_state.set(GameState::Paused),
            GameState::Paused => game_state.set(GameState::Playing),
            _ => {}
        }
    }
}

fn setup_shop_menu(
    mut commands: Commands,
    shop_registry: Res<ShopRegistry>,
    currency: Res<PlayerCurrency>,
) {
    // Shop UI container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            ShopMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Shop",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            // Currency display
            parent.spawn((
                TextBundle::from_section(
                    format!("Coins: {} | Gems: {} | Soul Shards: {}", 
                           currency.coins, currency.gems, currency.soul_shards),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                CurrencyDisplay,
            ));
            
            // Shop items
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|items_parent| {
                    for (item_id, item) in shop_registry.items.iter() {
                        items_parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(100.0),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                                    ..default()
                                },
                                ShopItemButton {
                                    item_id: item_id.clone(),
                                },
                            ))
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    &item.name,
                                    TextStyle {
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                                button.spawn(TextBundle::from_section(
                                    format!("Price: {}", item.price),
                                    TextStyle {
                                        font_size: 14.0,
                                        color: Color::srgb(0.5, 0.5, 0.5),
                                        ..default()
                                    },
                                ));
                            });
                    }
                });
            
            // Instructions
            parent.spawn(TextBundle::from_section(
                "Press S to close shop",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.5, 0.5, 0.5),
                    ..default()
                },
            ));
        });
}

fn update_shop_display(
    currency: Res<PlayerCurrency>,
    mut currency_text_q: Query<&mut Text, With<CurrencyDisplay>>,
) {
    if currency.is_changed() {
        for mut text in currency_text_q.iter_mut() {
            text.0 = format!(
                "Coins: {} | Gems: {} | Soul Shards: {}", 
                currency.coins, currency.gems, currency.soul_shards
            );
        }
    }
}

fn cleanup_shop_menu(
    mut commands: Commands,
    shop_menu_q: Query<Entity, With<ShopMenu>>,
) {
    for entity in shop_menu_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
