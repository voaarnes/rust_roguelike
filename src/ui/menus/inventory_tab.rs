use bevy::prelude::*;
use crate::systems::loot::{CollectedLoot, Equipment, Rarity};

pub fn render_inventory_tab(
    parent: &mut ChildBuilder,
    collected_loot: &CollectedLoot,
) {
    // Header
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(60.0),
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).with_children(|header| {
        header.spawn((
            Text::new("🎒 Inventory & Loot"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(0.8, 0.4, 0.8)),
        ));
        
        // Loot statistics
        header.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.0),
            ..default()
        }).with_children(|stats| {
            for (rarity, count) in &collected_loot.total_items {
                stats.spawn((
                    Text::new(format!("{:?}: {}", rarity, count)),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(rarity.color()),
                ));
            }
        });
    });
    
    // Equipment grid
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        ..default()
    }).with_children(|grid| {
        for equipment in &collected_loot.equipment {
            spawn_equipment_card(grid, equipment);
        }
        
        // Empty slots
        for _ in collected_loot.equipment.len()..20 {
            spawn_empty_slot(grid);
        }
    });
}

fn spawn_equipment_card(parent: &mut ChildBuilder, equipment: &Equipment) {
    parent.spawn(Node {
        width: Val::Px(120.0),
        height: Val::Px(120.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
    }).with_child(
        BackgroundColor(Color::srgb(0.2, 0.3, 0.4))
    ).with_children(|card| {
        card.spawn((
            Text::new(&equipment.name),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        card.spawn((
            Text::new(format!("{:?}", equipment.slot)),
            TextFont { font_size: 10.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
        
        // Stats preview
        for (stat, value) in equipment.stats.iter().take(2) {
            card.spawn((
                Text::new(format!("+{} {:?}", value, stat)),
                TextFont { font_size: 10.0, ..default() },
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
            ));
        }
    });
}

fn spawn_empty_slot(parent: &mut ChildBuilder) {
    parent.spawn(Node {
        width: Val::Px(120.0),
        height: Val::Px(120.0),
        ..default()
    }).with_child(
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.3))
    );
}
