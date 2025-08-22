use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PowerUpDisplayPlugin;

impl Plugin for PowerUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_powerup_display)
            .add_systems(Update, update_powerup_display);
    }
}

#[derive(Component)]
struct PowerUpSlotUI {
    slot_index: usize,
}

fn setup_powerup_display(mut commands: Commands) {
    // Create UI container for power-up slots
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(60.0),
                left: Val::Px(10.0),
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            for i in 0..4 {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8)),
                        ..default()
                    },
                    PowerUpSlotUI { slot_index: i },
                ));
            }
        });
}

fn update_powerup_display(
    player_query: Query<&PowerUpSlots, With<crate::game::player::Player>>,
    mut slot_query: Query<(&PowerUpSlotUI, &mut BackgroundColor)>,
) {
    if let Ok(powerup_slots) = player_query.single() {
        for (slot_ui, mut bg_color) in slot_query.iter_mut() {
            if slot_ui.slot_index < powerup_slots.slots.len() {
                *bg_color = match powerup_slots.slots[slot_ui.slot_index] {
                    Some(PowerUpType::SpeedBoost) => BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.0)),
                    Some(PowerUpType::DamageBoost) => BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                    Some(PowerUpType::HealthBoost) => BackgroundColor(Color::linear_rgb(0.0, 0.0, 1.0)),
                    Some(PowerUpType::ShieldBoost) => BackgroundColor(Color::linear_rgb(1.0, 1.0, 0.0)),
                    None => BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8)),
                };
            }
        }
    }
}
