// src/ui/powerup_display.rs
use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};
use crate::game::player::Player;

pub struct PowerUpDisplayPlugin;

impl Plugin for PowerUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_powerup_ui)
           .add_systems(Update, update_powerup_display);
    }
}

#[derive(Component)]
pub struct PowerupUI;

#[derive(Component)]
struct PowerupSlotUI {
    slot_index: usize,
}

pub fn setup_powerup_ui(mut commands: Commands) {
    // Main container
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            // optional: give the container a transparent bg
            BackgroundColor(Color::Srgba(Srgba::new(0.0, 0.0, 0.0, 0.0))),
            PowerupUI,
        ))
        .with_children(|parent| {
            for i in 0..4 {
                parent.spawn((
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(48.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::new(0.1, 0.1, 0.1, 0.8))),
                    PowerupSlotUI { slot_index: i },
                ));
            }
        });
}

pub fn update_powerup_display(
    player_query: Query<&PowerUpSlots, With<Player>>,
    mut slot_query: Query<(&PowerupSlotUI, &mut BackgroundColor)>,
) {
    if let Ok(powerup_slots) = player_query.get_single() {
        for (slot_ui, mut bg_color) in slot_query.iter_mut() {
            if slot_ui.slot_index < powerup_slots.slots.len() {
                *bg_color = match powerup_slots.slots[slot_ui.slot_index] {
                    Some(PowerUpType::SpeedBoost)  => BackgroundColor(Color::LinearRgba(LinearRgba::new(0.0, 1.0, 0.0, 1.0))),
                    Some(PowerUpType::DamageBoost) => BackgroundColor(Color::LinearRgba(LinearRgba::new(1.0, 0.0, 0.0, 1.0))),
                    Some(PowerUpType::HealthBoost) => BackgroundColor(Color::LinearRgba(LinearRgba::new(0.0, 0.0, 1.0, 1.0))),
                    Some(PowerUpType::ShieldBoost) => BackgroundColor(Color::LinearRgba(LinearRgba::new(1.0, 1.0, 0.0, 1.0))),
                    None => BackgroundColor(Color::Srgba(Srgba::new(0.2, 0.2, 0.2, 0.8))),
                };
            }
        }
    }
}
