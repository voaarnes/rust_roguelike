use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PowerUpDisplayPlugin;

impl Plugin for PowerUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_powerup_display)
           .add_systems(Update, update_powerup_display_with_fifo);
    }
}

#[derive(Component)]
pub struct PowerUpSlotUI {
    pub slot_index: usize,
}


fn setup_powerup_display(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|parent| {
            for i in 0..3 {  // Changed from 4 to 3
                parent.spawn((
                    Node {
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8)),
                    PowerUpSlotUI { slot_index: i },
                ));
            }
        });
}

fn update_powerup_display_with_fifo(
    player_query: Query<&crate::entities::powerup::PowerUpSlots, With<crate::game::player::Player>>,
    mut slot_query: Query<(&PowerUpSlotUI, &mut BackgroundColor)>,
) {
    if let Ok(powerup_slots) = player_query.single() {
        let slots_vec = powerup_slots.get_slots_as_vec();
        
        for (slot_ui, mut bg_color) in slot_query.iter_mut() {
            if slot_ui.slot_index < slots_vec.len() {
                *bg_color = match slots_vec[slot_ui.slot_index] {
                    Some(crate::entities::powerup::PowerUpType::SpeedBoost) => BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.0)),
                    Some(crate::entities::powerup::PowerUpType::DamageBoost) => BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                    Some(crate::entities::powerup::PowerUpType::HealthBoost) => BackgroundColor(Color::linear_rgb(0.0, 0.0, 1.0)),
                    Some(crate::entities::powerup::PowerUpType::ShieldBoost) => BackgroundColor(Color::linear_rgb(1.0, 1.0, 0.0)),
                    None => BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8)),
                };
            }
        }
    }
}
