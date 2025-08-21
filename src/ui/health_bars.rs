
use bevy::prelude::*;
use crate::components::{Health, HealthBar};
use crate::resources::HealthBarSettings;
use crate::constants::{HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT, HEALTH_BAR_OFFSET};

pub fn update_health_bars(
    mut commands: Commands,
    health_query: Query<(Entity, &Health, &Transform), Changed<Health>>,
    mut bar_query: Query<(Entity, &HealthBar, &mut Transform, &mut Sprite), Without<Health>>,
    settings: Res<HealthBarSettings>,
) {
    if !settings.enabled {
        return;
    }
    
    for (entity, health, owner_transform) in health_query.iter() {
        let mut bar_found = false;
        
        for (bar_entity, health_bar, mut bar_transform, mut sprite) in bar_query.iter_mut() {
            if health_bar.owner == entity {
                bar_found = true;
                
                // Update position
                bar_transform.translation = owner_transform.translation + Vec3::new(0.0, HEALTH_BAR_OFFSET, 1.0);
                
                // Update width based on health percentage
                let health_percentage = health.percentage();
                sprite.custom_size = Some(Vec2::new(HEALTH_BAR_WIDTH * health_percentage, HEALTH_BAR_HEIGHT));
                
                // Update color based on health
                sprite.color = if health_percentage > 0.6 {
                    Color::srgb(0.0, 1.0, 0.0)
                } else if health_percentage > 0.3 {
                    Color::srgb(1.0, 1.0, 0.0)
                } else {
                    Color::srgb(1.0, 0.0, 0.0)
                };
                
                break;
            }
        }
        
        // Create health bar if it doesn't exist
        if !bar_found && health.percentage() < 1.0 {
            spawn_health_bar(&mut commands, entity, owner_transform.translation, health.percentage());
        }
    }
}

fn spawn_health_bar(commands: &mut Commands, owner: Entity, position: Vec3, health_percentage: f32) {
    // Background bar
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, HEALTH_BAR_OFFSET, 0.9)),
        HealthBar { owner },
    ));
    
    // Health bar
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH * health_percentage, HEALTH_BAR_HEIGHT)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, HEALTH_BAR_OFFSET, 1.0)),
        HealthBar { owner },
    ));
}

pub fn toggle_health_bars(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<HealthBarSettings>,
    mut bar_query: Query<&mut Visibility, With<HealthBar>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        settings.enabled = !settings.enabled;
        
        let visibility = if settings.enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        
        for mut bar_visibility in bar_query.iter_mut() {
            *bar_visibility = visibility;
        }
    }
}
