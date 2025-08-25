use bevy::prelude::*;
use crate::systems::prestige::{PrestigeSystem, MetaProgression, PrestigeEvent, PrestigeType};

#[derive(Component)]
pub struct PrestigeButton(PrestigeType);

pub fn render_prestige_tab(
    parent: &mut ChildBuilder,
    prestige: &PrestigeSystem,
    meta: &MetaProgression,
) {
    // Header
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(100.0),
        flex_direction: FlexDirection::Column,
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).with_children(|header| {
        header.spawn((
            Text::new("♾️ Prestige & Meta-Progression"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(1.0, 0.0, 0.5)),
        ));
        
        // Prestige stats
        header.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(30.0),
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        }).with_children(|stats| {
            stats.spawn((
                Text::new(format!("Prestige Level: {}", prestige.current_prestige)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
            stats.spawn((
                Text::new(format!("Prestige Points: {}", prestige.prestige_points)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(1.0, 0.843, 0.0)),
            ));
            stats.spawn((
                Text::new(format!("Legacy Points: {}", prestige.legacy_points)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.8, 0.0, 0.8)),
            ));
            stats.spawn((
                Text::new(format!("Ascension Shards: {}", prestige.ascension_shards)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.0, 0.8, 1.0)),
            ));
        });
    });
    
    // Prestige options
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(20.0),
        margin: UiRect::bottom(Val::Px(30.0)),
        ..default()
    }).with_children(|options| {
        // Standard Prestige
        spawn_prestige_option(
            options,
            PrestigeType::Standard,
            "Standard Prestige",
            "Reset progress for prestige points\nRequires: Wave 50 or Level 100",
            Color::srgb(0.5, 0.5, 1.0),
        );
        
        // Ascension
        spawn_prestige_option(
            options,
            PrestigeType::Ascension,
            "Ascension",
            "Harder reset for ascension shards\nRequires: Prestige 10",
            Color::srgb(1.0, 0.5, 0.0),
        );
        
        // Rebirth
        spawn_prestige_option(
            options,
            PrestigeType::Rebirth,
            "Rebirth",
            "Complete reset for legacy points\nRequires: Prestige 25",
            Color::srgb(1.0, 0.0, 0.5),
        );
    });
    
    // Meta upgrades
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        flex_direction: FlexDirection::Column,
        ..default()
    }).with_children(|upgrades| {
        upgrades.spawn((
            Text::new("Meta Upgrades"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        upgrades.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(10.0),
            margin: UiRect::top(Val::Px(15.0)),
            ..default()
        }).with_children(|grid| {
            for (id, upgrade) in &meta.permanent_upgrades {
                spawn_meta_upgrade_card(grid, upgrade);
            }
        });
    });
}

fn spawn_prestige_option(
    parent: &mut ChildBuilder,
    prestige_type: PrestigeType,
    name: &str,
    description: &str,
    color: Color,
) {
    parent.spawn((
        Button,
        Node {
            flex: 1.0,
            padding: UiRect::all(Val::Px(15.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(color.red(), color.green(), color.blue(), 0.2)),
        PrestigeButton(prestige_type),
    )).with_children(|option| {
        option.spawn((
            Text::new(name),
            TextFont { font_size: 20.0, ..default() },
            TextColor(color),
        ));
        
        option.spawn((
            Text::new(description),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

fn spawn_meta_upgrade_card(
    parent: &mut ChildBuilder,
    upgrade: &crate::systems::prestige::MetaUpgrade,
) {
    parent.spawn(Node {
        width: Val::Px(200.0),
        padding: UiRect::all(Val::Px(10.0)),
        flex_direction: FlexDirection::Column,
        ..default()
    }).with_child(
        BackgroundColor(Color::srgb(0.15, 0.15, 0.2))
    ).with_children(|card| {
        card.spawn((
            Text::new(&upgrade.name),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        card.spawn((
            Text::new(&upgrade.description),
            TextFont { font_size: 11.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
        
        card.spawn((
            Text::new(format!("Level: {}/{}", upgrade.current_level, upgrade.max_level)),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::srgb(1.0, 0.843, 0.0)),
        ));
    });
}
