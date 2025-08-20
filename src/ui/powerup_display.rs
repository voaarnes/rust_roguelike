use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PowerUpDisplayPlugin;

impl Plugin for PowerUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_powerup_ui)
            .add_systems(Update, update_powerup_display);
    }
}

#[derive(Component)]
struct PowerUpSlotUI {
    slot_index: usize,
}

#[derive(Component)]
struct PowerUpUIRoot;

fn setup_powerup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        PowerUpUIRoot,
    )).with_children(|parent| {
        for i in 0..3 {
            parent.spawn((
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                PowerUpSlotUI { slot_index: i },
            ));
        }
    });
}

fn update_powerup_display(
    powerup_slots: Res<PowerUpSlots>,
    mut slot_query: Query<(&PowerUpSlotUI, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (slot_ui, mut bg_color, mut border_color) in slot_query.iter_mut() {
        if let Some(power_type) = powerup_slots.slots.get(slot_ui.slot_index) {
            let color = get_fruit_color(*power_type);
            *bg_color = BackgroundColor(color.with_alpha(0.6));
            *border_color = BorderColor(color);
        } else {
            *bg_color = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3));
            *border_color = BorderColor(Color::srgba(0.8, 0.8, 0.8, 0.5));
        }
    }
}

fn get_fruit_color(power_type: PowerUpType) -> Color {
    match power_type {
        PowerUpType::Strawberry => Color::srgb(1.0, 0.2, 0.3),
        PowerUpType::Pear => Color::srgb(0.7, 1.0, 0.2),
        PowerUpType::Mango => Color::srgb(1.0, 0.7, 0.1),
        PowerUpType::Apple => Color::srgb(0.9, 0.1, 0.1),
        PowerUpType::Orange => Color::srgb(1.0, 0.5, 0.0),
        PowerUpType::Grape => Color::srgb(0.6, 0.2, 0.8),
        PowerUpType::Banana => Color::srgb(1.0, 1.0, 0.2),
        PowerUpType::Cherry => Color::srgb(0.8, 0.1, 0.2),
    }
}
