use bevy::prelude::*;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PowerUpDisplayPlugin;

impl Plugin for PowerUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_powerup_display)
           .add_systems(Update, update_powerup_display);
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
            for i in 0..3 {  // Changed to 3 slots
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

fn update_powerup_display(
    player_query: Query<&PowerUpSlots, With<crate::game::player::Player>>,
    mut slot_query: Query<(&PowerUpSlotUI, &mut BackgroundColor)>,
) {
    if let Ok(powerup_slots) = player_query.single() {
        let slots_vec = powerup_slots.get_slots_as_vec();
        
        for (slot_ui, mut bg_color) in slot_query.iter_mut() {
            if slot_ui.slot_index < slots_vec.len() {
                *bg_color = match slots_vec[slot_ui.slot_index] {
                    Some(PowerUpType::SpeedBoost) => BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),    // Red for strawberry
                    Some(PowerUpType::DamageBoost) => BackgroundColor(Color::linear_rgb(1.0, 0.5, 0.0)),    // Orange for mango/pineapple
                    Some(PowerUpType::HealthBoost) => BackgroundColor(Color::linear_rgb(1.0, 0.65, 0.0)),   // Orange for apple/carrot
                    Some(PowerUpType::ShieldBoost) => BackgroundColor(Color::linear_rgb(1.0, 1.0, 0.0)),    // Yellow for coconut
                    None => BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8)),
                };
            } else {
                *bg_color = BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8));
            }
        }
    }
}
