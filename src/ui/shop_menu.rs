use bevy::prelude::*;
use crate::systems::shop::{PlayerCurrency, ShopInventory, PurchaseEvent};
use crate::core::state::GameState;

pub struct ShopMenuPlugin;

impl Plugin for ShopMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                shop_menu_input,
                update_shop_display,
                handle_shop_button_clicks,
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
    shop_inventory: Res<ShopInventory>,
    currency: Res<PlayerCurrency>,
) {
    // Shop UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                top: Val::Percent(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ShopMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Shop"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
            ));
            
            // Currency display
            parent.spawn((
                Text::new(format!("Coins: {} | Gems: {} | Soul Shards: {}", 
                           currency.coins, currency.gems, currency.soul_shards)),
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                CurrencyDisplay,
            ));
            
            // Shop items
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                ))
                .with_children(|items_parent| {
                    for item in shop_inventory.items.iter() {
                        items_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(200.0),
                                    height: Val::Px(100.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    padding: UiRect::all(Val::Px(10.0)),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                ShopItemButton {
                                    item_id: item.id.clone(),
                                },
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(&item.name),
                                    TextColor(Color::WHITE),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                ));
                                button.spawn((
                                    Text::new(format!("Cost: {}", item.cost)),
                                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                ));
                            });
                    }
                });
            
            // Instructions
            parent.spawn((
                Text::new("Press S to close shop"),
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                TextFont {
                    font_size: 16.0,
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
            **text = format!(
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
        commands.entity(entity).despawn();
    }
}

fn handle_shop_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &ShopItemButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut purchase_events: EventWriter<PurchaseEvent>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
) {
    let Ok(player_entity) = player_q.get_single() else { return };
    
    for (interaction, shop_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            purchase_events.send(PurchaseEvent {
                item_id: shop_button.item_id.clone(),
                player: player_entity,
            });
        }
    }
}
