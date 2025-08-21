use bevy::prelude::*;
use crate::core::state::GameStats;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameStats>()
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct WaveText;

fn setup_hud(mut commands: Commands) {
    // Score display
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
    
    // Health display  
    commands.spawn((
        TextBundle::from_section(
            "Health: 100/100",
            TextStyle {
                font_size: 24.0,
                color: Color::linear_rgb(0.0, 1.0, 0.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HealthText,
    ));
    
    // Wave display
    commands.spawn((
        TextBundle::from_section(
            "Wave: 1",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        WaveText,
    ));
}

fn update_hud(
    mut score_q: Query<&mut Text, (With<ScoreText>, Without<HealthText>, Without<WaveText>)>,
    mut health_q: Query<&mut Text, (With<HealthText>, Without<ScoreText>, Without<WaveText>)>,
    mut wave_q: Query<&mut Text, (With<WaveText>, Without<ScoreText>, Without<HealthText>)>,
    player_q: Query<&crate::game::combat::Health, With<crate::game::player::Player>>,
    stats: Res<GameStats>,
    wave_manager: Res<crate::game::spawning::WaveManager>,
) {
    for mut text in score_q.iter_mut() {
        *text = Text::from_section(
            format!("Score: {}", stats.score),
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        );
    }
    
    if let Ok(health) = player_q.get_single() {
        for mut text in health_q.iter_mut() {
            let color = if health.percentage() > 0.6 {
                Color::linear_rgb(0.0, 1.0, 0.0)
            } else if health.percentage() > 0.3 {
                Color::linear_rgb(1.0, 1.0, 0.0)
            } else {
                Color::linear_rgb(1.0, 0.0, 0.0)
            };
            
            *text = Text::from_section(
                format!("Health: {}/{}", health.current, health.max),
                TextStyle {
                    font_size: 24.0,
                    color,
                    ..default()
                },
            );
        }
    }
    
    for mut text in wave_q.iter_mut() {
        *text = Text::from_section(
            format!("Wave: {}", wave_manager.current_wave),
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        );
    }
}
