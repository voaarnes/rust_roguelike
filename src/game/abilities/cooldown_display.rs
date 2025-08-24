use bevy::prelude::*;
use crate::game::player::Player;
use super::ActiveAbilities;

pub struct CooldownDisplayPlugin;

impl Plugin for CooldownDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_cooldowns);
    }
}

#[derive(Component)]
pub struct CooldownBar {
    pub ability_type: AbilityBarType,
}

#[derive(Clone, Copy)]
pub enum AbilityBarType {
    Head,
    Torso,
    Legs,
}

fn display_cooldowns(
    player_q: Query<&ActiveAbilities, With<Player>>,
    mut gizmos: Gizmos,
    camera_q: Query<&Transform, With<Camera>>,
) {
    let Ok(abilities) = player_q.single() else { return };
    let Ok(camera_transform) = camera_q.single() else { return };
    
    let base_y = camera_transform.translation.y - 280.0;
    let base_x = camera_transform.translation.x - 160.0;
    
    // Head ability cooldown
    if let Some(ref ability) = abilities.head_ability {
        let progress = 1.0 - ability.cooldown_timer.fraction();
        draw_cooldown_bar(&mut gizmos, Vec2::new(base_x, base_y + 60.0), progress, Color::srgb(1.0, 0.8, 0.0));
    }
    
    // Torso ability cooldown
    if let Some(ref ability) = abilities.torso_ability {
        let progress = 1.0 - ability.cooldown_timer.fraction();
        draw_cooldown_bar(&mut gizmos, Vec2::new(base_x, base_y + 40.0), progress, Color::srgb(0.0, 1.0, 0.8));
    }
    
    // Legs ability cooldown
    if let Some(ref ability) = abilities.legs_ability {
        let progress = 1.0 - ability.cooldown_timer.fraction();
        draw_cooldown_bar(&mut gizmos, Vec2::new(base_x, base_y + 20.0), progress, Color::srgb(0.8, 0.0, 1.0));
    }
}

fn draw_cooldown_bar(gizmos: &mut Gizmos, position: Vec2, progress: f32, color: Color) {
    let width = 100.0;
    let height = 8.0;
    
    // Background
    gizmos.rect_2d(
        position + Vec2::new(width / 2.0, 0.0),
        Vec2::new(width, height),
        Color::srgba(0.2, 0.2, 0.2, 0.8),
    );
    
    // Fill
    if progress > 0.0 {
        gizmos.rect_2d(
            position + Vec2::new(width * progress / 2.0, 0.0),
            Vec2::new(width * progress, height - 2.0),
            color,
        );
    }
}
