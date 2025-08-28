use bevy::prelude::*;
use bevy::ui::*;
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
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(20.0),
                        top: Val::Px(20.0),
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.5, 0.0, 0.9)),
                    AchievementNotification {
                        timer: Timer::from_seconds(3.0, TimerMode::Once),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Achievement Unlocked!"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new(&achievement.name),
                        TextColor(Color::srgb(1.0, 1.0, 0.0)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new(&achievement.description),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TextFont {
                            font_size: 14.0,
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
            commands.entity(entity).despawn();
        }
    }
}
