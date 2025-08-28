pub mod main_menu;
pub mod hud;
pub mod health_bars;
pub mod minimap;
pub mod powerup_display;  // Re-enabled after fixing Bevy 0.16 compatibility
pub mod ability_display;  // Re-enabled after fixing Bevy 0.16 compatibility
pub mod shop_menu;        // Re-enabled after fixing Bevy 0.16 compatibility
pub mod talent_menu;      // Re-enabled after fixing Bevy 0.16 compatibility
pub mod achievement_display; // Re-enabled after fixing Bevy 0.16 compatibility

// New advanced UI modules
pub mod components;
pub mod menus;
pub mod overlays;

// UI modules temporarily disabled for compilation testing

// Legacy modules (disabled)
// pub mod pause_menu;  // Replaced by menus::main_game_menu

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Core UI plugins
            hud::HUDPlugin,
            main_menu::MainMenuPlugin,
            health_bars::HealthBarPlugin,
            minimap::MinimapPlugin,
            powerup_display::PowerUpDisplayPlugin,  // Re-enabled
            ability_display::AbilityDisplayPlugin,  // Re-enabled
            // Integrated systems UI
            shop_menu::ShopMenuPlugin,
            talent_menu::TalentMenuPlugin,
            achievement_display::AchievementDisplayPlugin,
            
            // Advanced menu system (replaces pause_menu)
            menus::main_game_menu::MainGameMenuPlugin,
            
            // Enhanced HUD
            components::game_hud::GameHudPlugin,
        ));
    }
}
