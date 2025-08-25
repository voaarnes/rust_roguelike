pub mod main_menu;
pub mod pause_menu;
pub mod hud;
pub mod health_bars;
pub mod minimap;
pub mod powerup_display;
pub mod ability_display;
// Temporarily disabled for Bevy 0.16 compatibility
// pub mod shop_menu;
// pub mod achievement_display;
// pub mod talent_menu;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hud::HUDPlugin,
            main_menu::MainMenuPlugin,
            pause_menu::PauseMenuPlugin,
            health_bars::HealthBarPlugin,
            minimap::MinimapPlugin,
            powerup_display::PowerUpDisplayPlugin,
            ability_display::AbilityDisplayPlugin,
            // shop_menu::ShopMenuPlugin,
            // achievement_display::AchievementDisplayPlugin,
            // talent_menu::TalentMenuPlugin,
        ));
    }
}
