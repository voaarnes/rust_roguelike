use bevy::prelude::*;
use crate::systems::shop::{ShopInventory, PlayerCurrency, ShopItem, PurchaseEvent};

#[derive(Component)]
pub struct ShopItemCard {
    pub item_id: String,
}

pub fn render_shop_tab(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    shop: &ShopInventory,
    currency: &PlayerCurrency,
) {
    // Header with currency
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(80.0),
        flex_direction: FlexDirection::Column,
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).with_children(|header| {
        header.spawn((
            Text::new("🛒 Shop"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(1.0, 0.843, 0.0)),
        ));
        
        header.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(30.0),
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        }).with_children(|currencies| {
            currencies.spawn((
                Text::new(format!("💰 Coins: {}", currency.coins)),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(1.0, 0.843, 0.0)),
            ));
            currencies.spawn((
                Text::new(format!("💎 Gems: {}", currency.gems)),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 1.0)),
            ));
            currencies.spawn((
                Text::new(format!("👻 Soul Shards: {}", currency.soul_shards)),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.8, 0.0, 0.8)),
            ));
        });
    });
    
    // Shop items grid
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        row_gap: Val::Px(15.0),
        column_gap: Val::Px(15.0),
        ..default()
    }).with_children(|grid| {
        for item in &shop.items {
            spawn_shop_item_card(grid, item);
        }
    });
}

fn spawn_shop_item_card(parent: &mut impl bevy::hierarchy::BuildChildren, item: &ShopItem) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(180.0),
            height: Val::Px(220.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(match item.tier {
            crate::systems::shop::ItemTier::Common => Color::srgb(0.2, 0.2, 0.2),
            crate::systems::shop::ItemTier::Uncommon => Color::srgb(0.1, 0.3, 0.1),
            crate::systems::shop::ItemTier::Rare => Color::srgb(0.1, 0.2, 0.4),
            crate::systems::shop::ItemTier::Epic => Color::srgb(0.3, 0.1, 0.4),
            crate::systems::shop::ItemTier::Legendary => Color::srgb(0.4, 0.3, 0.1),
        }),
        ShopItemCard { item_id: item.id.clone() },
    )).with_children(|card| {
        // Item name
        card.spawn((
            Text::new(&item.name),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // Item description
        card.spawn(Node {
            flex_grow: 1.0,
            margin: UiRect::vertical(Val::Px(5.0)),
            ..default()
        }).with_children(|desc| {
            desc.spawn((
                Text::new(&item.description),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
        
        // Price
        card.spawn((
            Text::new(format!("Cost: {}", item.cost)),
            TextFont { font_size: 14.0, ..default() },
            TextColor(match item.currency_type {
                crate::systems::shop::CurrencyType::Coins => Color::srgb(1.0, 0.843, 0.0),
                crate::systems::shop::CurrencyType::Gems => Color::srgb(0.5, 0.5, 1.0),
                crate::systems::shop::CurrencyType::SoulShards => Color::srgb(0.8, 0.0, 0.8),
            }),
        ));
        
        // Stock
        if item.stock > 0 {
            card.spawn((
                Text::new(format!("Stock: {}", item.stock)),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(1.0, 0.5, 0.5)),
            ));
        }
    });
}
