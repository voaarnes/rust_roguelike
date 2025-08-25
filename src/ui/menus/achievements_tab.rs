use bevy::prelude::*;
use crate::systems::achievements::{PlayerAchievements, AchievementRegistry, AchievementCategory};

pub fn render_achievements_tab(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    registry: &AchievementRegistry,
    player_achievements: &PlayerAchievements,
) {
    // Header
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(80.0),
        flex_direction: FlexDirection::Column,
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).with_children(|header| {
        header.spawn((
            Text::new("🏆 Achievements"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(1.0, 0.5, 0.0)),
        ));
        
        let unlocked_count = player_achievements.unlocked.len();
        let total_count = registry.achievements.len();
        let completion = (unlocked_count as f32 / total_count as f32 * 100.0) as u32;
        
        header.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(30.0),
            ..default()
        }).with_children(|stats| {
            stats.spawn((
                Text::new(format!("Unlocked: {}/{}", unlocked_count, total_count)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
            stats.spawn((
                Text::new(format!("{}% Complete", completion)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
            ));
            stats.spawn((
                Text::new(format!("Points: {}", player_achievements.total_points)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(1.0, 0.843, 0.0)),
            ));
        });
    });
    
    // Achievement categories
    let categories = [
        AchievementCategory::Progression,
        AchievementCategory::Combat,
        AchievementCategory::Collection,
        AchievementCategory::Challenge,
        AchievementCategory::Secret,
    ];
    
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(20.0),
        ..default()
    }).with_children(|content| {
        for category in categories {
            spawn_achievement_category(content, category, registry, player_achievements);
        }
    });
}

fn spawn_achievement_category(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    category: AchievementCategory,
    registry: &AchievementRegistry,
    player_achievements: &PlayerAchievements,
) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }).with_children(|cat| {
        // Category header
        cat.spawn((
            Text::new(format!("{:?}", category)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 1.0)),
        ));
        
        // Achievement grid
        cat.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(10.0),
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        }).with_children(|grid| {
            for (id, achievement) in &registry.achievements {
                if achievement.category != category { continue; }
                
                let is_unlocked = player_achievements.unlocked.get(id).copied().unwrap_or(false);
                let progress = player_achievements.progress.get(id).copied().unwrap_or(0);
                
                spawn_achievement_card(grid, achievement, is_unlocked, progress);
            }
        });
    });
}

fn spawn_achievement_card(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    achievement: &crate::systems::achievements::Achievement,
    is_unlocked: bool,
    progress: u32,
) {
    parent.spawn(Node {
        width: Val::Px(160.0),
        height: Val::Px(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
    }).with_child(
        BackgroundColor(if is_unlocked {
            match achievement.tier {
                crate::systems::achievements::AchievementTier::Bronze => Color::srgb(0.5, 0.3, 0.1),
                crate::systems::achievements::AchievementTier::Silver => Color::srgb(0.4, 0.4, 0.4),
                crate::systems::achievements::AchievementTier::Gold => Color::srgb(0.6, 0.5, 0.1),
                crate::systems::achievements::AchievementTier::Platinum => Color::srgb(0.3, 0.5, 0.6),
                crate::systems::achievements::AchievementTier::Diamond => Color::srgb(0.4, 0.2, 0.6),
            }
        } else {
            Color::srgb(0.1, 0.1, 0.1)
        })
    ).with_children(|card| {
        // Achievement name
        card.spawn((
            Text::new(&achievement.name),
            TextFont { font_size: 14.0, ..default() },
            TextColor(if is_unlocked { Color::WHITE } else { Color::srgb(0.5, 0.5, 0.5) }),
        ));
        
        // Description
        card.spawn((
            Text::new(&achievement.description),
            TextFont { font_size: 11.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
        
        // Progress bar if not complete
        if !is_unlocked {
            // Show progress indicator
            card.spawn((
                Text::new(format!("Progress: {}", progress)),
                TextFont { font_size: 10.0, ..default() },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
            ));
        }
    });
}
