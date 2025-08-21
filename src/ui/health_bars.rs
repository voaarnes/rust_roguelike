use bevy::prelude::*;
use crate::game::combat::Health; // <- adjust if Health lives elsewhere

// ---- local settings & constants (self-contained) ----
#[derive(Resource)]
pub struct HealthBarSettings {
    pub enabled: bool,
}
impl Default for HealthBarSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

// You can tweak these if you had different values before.
const HEALTH_BAR_WIDTH: f32 = 24.0;
const HEALTH_BAR_HEIGHT: f32 = 3.0;
const HEALTH_BAR_OFFSET: f32 = 18.0;

// Marker for both foreground (colored) and background bars
#[derive(Component)]
pub struct HealthBar {
    pub owner: Entity,
    pub foreground: bool, // true = colored (health), false = background (grey)
}

// ---- plugin ----
pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HealthBarSettings>()
            .add_systems(Update, (update_health_bars, toggle_health_bars));
    }
}

// ---- systems ----
pub fn update_health_bars(
    mut commands: Commands,
    // when a unit's health changes, (re)sync its bar(s)
    health_query: Query<(Entity, &Health, &Transform), Changed<Health>>,
    mut bar_query: Query<(&mut Transform, &mut Sprite, &HealthBar), Without<Health>>,
    settings: Res<HealthBarSettings>,
) {
    if !settings.enabled {
        return;
    }

    for (owner_entity, health, owner_tf) in &health_query {
        let health_pct = health.percentage();

        // find an existing *foreground* bar for this owner
        let mut had_foreground = false;

        for (mut bar_tf, mut sprite, hb) in bar_query.iter_mut() {
            if hb.owner == owner_entity && hb.foreground {
                had_foreground = true;

                // position bar above the owner
                bar_tf.translation = owner_tf.translation + Vec3::new(0.0, HEALTH_BAR_OFFSET, 1.0);

                // update width and color
                sprite.custom_size = Some(Vec2::new(
                    HEALTH_BAR_WIDTH * health_pct,
                    HEALTH_BAR_HEIGHT,
                ));

                sprite.color = if health_pct > 0.6 {
                    Color::srgb(0.0, 1.0, 0.0)
                } else if health_pct > 0.3 {
                    Color::srgb(1.0, 1.0, 0.0)
                } else {
                    Color::srgb(1.0, 0.0, 0.0)
                };

                break;
            }
        }

        // (Re)create bars if missing and health is not full
        if !had_foreground && health_pct < 1.0 {
            spawn_health_bar(&mut commands, owner_entity, owner_tf.translation, health_pct);
        }
    }
}

fn spawn_health_bar(commands: &mut Commands, owner: Entity, pos: Vec3, health_pct: f32) {
    // background (grey) behind the colored bar
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
            ..default()
        },
        Transform::from_translation(pos + Vec3::new(0.0, HEALTH_BAR_OFFSET, 0.9)),
        Visibility::Visible,
        HealthBar { owner, foreground: false },
    ));

    // foreground (colored) health
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH * health_pct, HEALTH_BAR_HEIGHT)),
            ..default()
        },
        Transform::from_translation(pos + Vec3::new(0.0, HEALTH_BAR_OFFSET, 1.0)),
        Visibility::Visible,
        HealthBar { owner, foreground: true },
    ));
}

pub fn toggle_health_bars(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<HealthBarSettings>,
    mut bar_q: Query<&mut Visibility, With<HealthBar>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        settings.enabled = !settings.enabled;
        let vis = if settings.enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        for mut v in &mut bar_q {
            *v = vis;
        }
    }
}
