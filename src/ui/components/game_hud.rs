use bevy::prelude::*;
use crate::systems::shop::PlayerCurrency;
use crate::systems::combo::ComboTracker;
use crate::systems::quests::ActiveQuests;
use crate::game::player::Player;
use crate::game::combat::Health;
use crate::game::spawning::WaveManager;
use crate::core::state::GameStats;

pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_game_hud)
            .add_systems(Update, (
                update_currency_display,
                update_combo_display,
                update_quest_tracker,
                update_wave_info,
            ));
    }
}

#[derive(Component)]
struct CurrencyDisplayUI;

#[derive(Component)]
struct ComboDisplayUI;

#[derive(Component)]
struct QuestTrackerUI;

#[derive(Component)]
struct WaveInfoUI;

fn setup_game_hud(mut commands: Commands) {
    // Top Bar Container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(60.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        // Currency Display (Left)
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            CurrencyDisplayUI,
        )).with_children(|currency_parent| {
            // Coins
            currency_parent.spawn((
                Text::new("💰 0"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(1.0, 0.843, 0.0)),
            ));
            // Gems
            currency_parent.spawn((
                Text::new("💎 0"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 1.0)),
            ));
            // Soul Shards
            currency_parent.spawn((
                Text::new("👻 0"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.8, 0.0, 0.8)),
            ));
        });

        // Wave Info (Center)
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            WaveInfoUI,
        )).with_children(|wave_parent| {
            wave_parent.spawn((
                Text::new("Wave 1"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            wave_parent.spawn((
                Text::new("Enemies: 0/0"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });

        // Combo Display (Right)
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                ..default()
            },
            ComboDisplayUI,
        )).with_children(|combo_parent| {
            combo_parent.spawn((
                Text::new(""),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
            combo_parent.spawn((
                Text::new(""),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(1.0, 0.5, 0.0)),
            ));
        });
    });

    // Quest Tracker (Right Side)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(80.0),
            width: Val::Px(250.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        QuestTrackerUI,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Active Quests"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(1.0, 0.843, 0.0)),
        ));
    });
}

fn update_currency_display(
    currency: Res<PlayerCurrency>,
    mut writer: TextUiWriter,
    query: Query<&Children, With<CurrencyDisplayUI>>,
    text_query: Query<Entity, With<Text>>,
) {
    if !currency.is_changed() { return; }
    
    if let Ok(children) = query.single() {
        let texts: Vec<Entity> = children.iter()
            .filter_map(|&child| text_query.get(child).ok())
            .collect();
        
        if texts.len() >= 3 {
            *writer.text(texts[0], 0) = format!("💰 {}", currency.coins);
            *writer.text(texts[1], 0) = format!("💎 {}", currency.gems);
            *writer.text(texts[2], 0) = format!("👻 {}", currency.soul_shards);
        }
    }
}

fn update_combo_display(
    combo: Res<ComboTracker>,
    mut writer: TextUiWriter,
    query: Query<&Children, With<ComboDisplayUI>>,
    text_query: Query<Entity, With<Text>>,
) {
    if let Ok(children) = query.single() {
        let texts: Vec<Entity> = children.iter()
            .filter_map(|&child| text_query.get(child).ok())
            .collect();
        
        if texts.len() >= 2 {
            if combo.current_combo > 0 {
                *writer.text(texts[0], 0) = format!("{}x COMBO", combo.current_combo);
                *writer.text(texts[1], 0) = format!("×{:.1} multiplier", combo.combo_multiplier);
                
                // Update color based on combo tier
                let color = match combo.current_combo {
                    0..=9 => Color::WHITE,
                    10..=24 => Color::srgb(0.5, 1.0, 0.5),
                    25..=49 => Color::srgb(0.0, 0.5, 1.0),
                    50..=99 => Color::srgb(0.7, 0.0, 1.0),
                    _ => Color::srgb(1.0, 0.843, 0.0),
                };
                writer.color(texts[0], 0).0 = color;
            } else {
                *writer.text(texts[0], 0) = "".to_string();
                *writer.text(texts[1], 0) = "".to_string();
            }
        }
    }
}

fn update_quest_tracker(
    quests: Res<ActiveQuests>,
    mut writer: TextUiWriter,
    query: Query<Entity, With<QuestTrackerUI>>,
    mut commands: Commands,
) {
    // Implementation for quest tracker updates
}

fn update_wave_info(
    wave: Res<WaveManager>,
    stats: Res<GameStats>,
    mut writer: TextUiWriter,
    query: Query<&Children, With<WaveInfoUI>>,
    text_query: Query<Entity, With<Text>>,
) {
    if let Ok(children) = query.single() {
        let texts: Vec<Entity> = children.iter()
            .filter_map(|&child| text_query.get(child).ok())
            .collect();
        
        if texts.len() >= 2 {
            *writer.text(texts[0], 0) = format!("Wave {}", wave.current_wave);
            *writer.text(texts[1], 0) = format!("Enemies: {}/{}", 
                wave.enemies_remaining, wave.enemies_per_wave);
        }
    }
}
