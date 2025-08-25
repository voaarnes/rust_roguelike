use bevy::prelude::*;
use crate::systems::quests::{ActiveQuests, QuestManager, QuestStatus};

pub fn render_quest_tab(
    parent: &mut ChildBuilder,
    active_quests: &ActiveQuests,
    quest_manager: &QuestManager,
) {
    // Header
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(60.0),
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).with_children(|header| {
        header.spawn((
            Text::new("📋 Quests & Challenges"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 1.0)),
        ));
    });
    
    // Quest sections
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(20.0),
        ..default()
    }).with_children(|content| {
        // Daily Quests
        content.spawn(Node {
            flex: 1.0,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        }).with_child(
            BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.5))
        ).with_children(|daily| {
            daily.spawn((
                Text::new("Daily Quests"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(1.0, 0.843, 0.0)),
            ));
            
            for quest in &active_quests.daily_quests {
                spawn_quest_item(daily, quest);
            }
        });
        
        // Wave Challenges
        content.spawn(Node {
            flex: 1.0,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        }).with_child(
            BackgroundColor(Color::srgba(0.2, 0.1, 0.1, 0.5))
        ).with_children(|challenges| {
            challenges.spawn((
                Text::new("Wave Challenges"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
            
            for quest in &active_quests.wave_challenges {
                spawn_quest_item(challenges, quest);
            }
        });
    });
}

fn spawn_quest_item(parent: &mut ChildBuilder, quest: &crate::systems::quests::ActiveQuest) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.0)),
        margin: UiRect::vertical(Val::Px(5.0)),
        ..default()
    }).with_child(
        BackgroundColor(if quest.completed {
            Color::srgb(0.1, 0.3, 0.1)
        } else {
            Color::srgb(0.15, 0.15, 0.15)
        })
    ).with_children(|item| {
        // Quest name
        item.spawn((
            Text::new(&quest.quest.name),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // Quest description
        item.spawn((
            Text::new(&quest.quest.description),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
        
        // Objectives
        for (idx, objective) in quest.quest.objectives.iter().enumerate() {
            let progress = quest.progress.get(&idx).copied().unwrap_or(0);
            item.spawn((
                Text::new(format!("• {:?} - {}", objective, progress)),
                TextFont { font_size: 11.0, ..default() },
                TextColor(if quest.completed {
                    Color::srgb(0.5, 1.0, 0.5)
                } else {
                    Color::srgb(0.8, 0.8, 0.8)
                }),
            ));
        }
        
        // Rewards
        item.spawn((
            Text::new(format!("Rewards: {} XP", quest.quest.rewards.experience)),
            TextFont { font_size: 11.0, ..default() },
            TextColor(Color::srgb(1.0, 0.843, 0.0)),
        ));
    });
}
