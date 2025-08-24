use bevy::prelude::*;
use crate::game::abilities::{ActiveAbilities, AbilityRegistry};
use crate::game::player::Player;

pub struct AbilityDisplayPlugin;

impl Plugin for AbilityDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ability_display)
            .add_systems(Update, update_ability_display);
    }
}

#[derive(Component)]
struct AbilityDisplayUI;

#[derive(Component)]
struct HeadAbilityDisplay;

#[derive(Component)]
struct TorsoAbilityDisplay;

#[derive(Component)]
struct LegsAbilityDisplay;

fn setup_ability_display(mut commands: Commands) {
    // Container for ability display
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(60.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
            AbilityDisplayUI,
        ))
        .with_children(|parent| {
            // Head ability
            parent.spawn((
                Text::new("Head: None"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(1.0, 0.8, 0.0)),
                HeadAbilityDisplay,
            ));
            
            // Torso ability
            parent.spawn((
                Text::new("Torso: None"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.0, 1.0, 0.8)),
                TorsoAbilityDisplay,
            ));
            
            // Legs ability
            parent.spawn((
                Text::new("Legs: None"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.8, 0.0, 1.0)),
                LegsAbilityDisplay,
            ));
        });
}

fn update_ability_display(
    player_q: Query<&ActiveAbilities, With<Player>>,
    registry: Res<AbilityRegistry>,
    mut writer: TextUiWriter,
    head_q: Query<Entity, With<HeadAbilityDisplay>>,
    torso_q: Query<Entity, With<TorsoAbilityDisplay>>,
    legs_q: Query<Entity, With<LegsAbilityDisplay>>,
) {
    let Ok(abilities) = player_q.single() else { return };
    
    // Update head ability text
    if let Ok(entity) = head_q.single() {
        let text = if let Some(ref ability) = abilities.head_ability {
            if let Some(def) = registry.abilities.get(&ability.ability_id) {
                format!("Head: {}", def.name)
            } else {
                "Head: None".to_string()
            }
        } else {
            "Head: None".to_string()
        };
        *writer.text(entity, 0) = text;
    }
    
    // Update torso ability text
    if let Ok(entity) = torso_q.single() {
        let text = if let Some(ref ability) = abilities.torso_ability {
            if let Some(def) = registry.abilities.get(&ability.ability_id) {
                format!("Torso: {}", def.name)
            } else {
                "Torso: None".to_string()
            }
        } else {
            "Torso: None".to_string()
        };
        *writer.text(entity, 0) = text;
    }
    
    // Update legs ability text
    if let Ok(entity) = legs_q.single() {
        let text = if let Some(ref ability) = abilities.legs_ability {
            if let Some(def) = registry.abilities.get(&ability.ability_id) {
                format!("Legs: {}", def.name)
            } else {
                "Legs: None".to_string()
            }
        } else {
            "Legs: None".to_string()
        };
        *writer.text(entity, 0) = text;
    }
}
