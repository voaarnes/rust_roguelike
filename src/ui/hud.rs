use bevy::prelude::*;
use crate::entities::player::PlayerStats;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct StatsText;

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Health: 100/100"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
            HealthText,
        ));
        
        parent.spawn((
            Text::new("Speed: 1.0x"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.2, 1.0, 0.2)),
            SpeedText,
        ));
        
        parent.spawn((
            Text::new("Defense: 0 | Crit: 0%"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            StatsText,
        ));
    });
}

fn update_hud(
    player_query: Query<&PlayerStats, With<crate::entities::player::Player>>,
    mut health_text: Query<&mut Text, (With<HealthText>, Without<SpeedText>, Without<StatsText>)>,
    mut speed_text: Query<&mut Text, (With<SpeedText>, Without<HealthText>, Without<StatsText>)>,
    mut stats_text: Query<&mut Text, (With<StatsText>, Without<HealthText>, Without<SpeedText>)>,
) {
    if let Ok(stats) = player_query.get_single() {
        for mut text in health_text.iter_mut() {
            let total_health = stats.get_total_health();
            **text = format!("Health: {}/{}", total_health, total_health);
        }
        
        for mut text in speed_text.iter_mut() {
            **text = format!("Speed: {:.1}x", stats.speed_multiplier);
        }
        
        for mut text in stats_text.iter_mut() {
            **text = format!(
                "Defense: {} | Crit: {:.0}% | Dodge: {:.0}%",
                stats.defense,
                stats.critical_chance * 100.0,
                stats.dodge_chance * 100.0
            );
        }
    }
}
