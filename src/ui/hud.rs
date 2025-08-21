use bevy::prelude::*;
use crate::core::state::GameStats;
use crate::game::spawning::WaveManager;
use crate::game::player::Player;
use crate::game::combat::Health;

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
    // Score (top-left)
    commands.spawn((
        Text::new("Score: 0"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));

    // Health (bottom-left)
    commands.spawn((
        Text::new("Health: 100/100"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::linear_rgb(0.0, 1.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        HealthText,
    ));

    // Wave (top-right)
    commands.spawn((
        Text::new("Wave: 1"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        WaveText,
    ));
}

fn update_hud(
    stats: Res<GameStats>,
    wave: Res<WaveManager>,
    player_health_q: Query<&Health, With<Player>>,
    // text updating is done via TextUiWriter in 0.15+
    mut writer: TextUiWriter,
    score_root: Query<Entity, With<ScoreText>>,
    wave_root: Query<Entity, With<WaveText>>,
    health_root: Query<Entity, With<HealthText>>,
) {
    if let Ok(root) = score_root.single() {
        *writer.text(root, 0) = format!("Score: {}", stats.score);
    }
    if let Ok(root) = wave_root.single() {
        *writer.text(root, 0) = format!("Wave: {}", wave.current_wave);
    }
    if let Ok(h) = player_health_q.single() {
        if let Ok(root) = health_root.single() {
            // update text
            *writer.text(root, 0) = format!("Health: {}/{}", h.current, h.max);

            // update color via writer to avoid the borrow conflict
            let c = if h.percentage() > 0.6 {
                Color::linear_rgb(0.0, 1.0, 0.0)
            } else if h.percentage() > 0.3 {
                Color::linear_rgb(1.0, 1.0, 0.0)
            } else {
                Color::linear_rgb(1.0, 0.0, 0.0)
            };
            writer.color(root, 0).0 = c; // TextColor is a tuple struct around Color
        }
    }
}
