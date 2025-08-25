use bevy::prelude::*;
use crate::systems::achievements::{PlayerAchievements, AchievementRegistry, AchievementUnlockedEvent};

pub struct AchievementDisplayPlugin;

impl Plugin for AchievementDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                show_achievement_notifications,
                update_achievement_progress_display,
            ));
    }
}

#[derive(Component)]
pub struct AchievementNotification {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AchievementProgressDisplay;

fn show_achievement_notifications(
    mut commands: Commands,
    mut events: EventReader<AchievementUnlockedEvent>,
    registry: Res<AchievementRegistry>,
) {
    for event in events.read() {
        if let Some(achievement) = registry.achievements.get(&event.achievement_id) {
            // Spawn achievement notification UI
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            right: Val::Px(20.0),
                            top: Val::Px(20.0),
                            width: Val::Px(300.0),
                            height: Val::Px(80.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.5, 0.0, 0.9).into(),
                        ..default()
                    },
                    AchievementNotification {
                        timer: Timer::from_seconds(3.0, TimerMode::Once),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Achievement Unlocked!",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        &achievement.name,
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(1.0, 1.0, 0.0),
                            ..default()
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        &achievement.description,
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        }
    }
}

fn update_achievement_progress_display(
    mut commands: Commands,
    time: Res<Time>,
    mut notification_q: Query<(Entity, &mut AchievementNotification)>,
) {
    for (entity, mut notification) in notification_q.iter_mut() {
        notification.timer.tick(time.delta());
        if notification.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
