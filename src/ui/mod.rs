pub mod main_menu;
pub mod hud;
pub mod health_bars;
pub mod minimap;
// pub mod powerup_display;  // Temporarily disabled
// pub mod ability_display;  // Temporarily disabled
// pub mod shop_menu;        // Temporarily disabled
// pub mod talent_menu;      // Temporarily disabled
// pub mod achievement_display; // Temporarily disabled

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
            // powerup_display::PowerUpDisplayPlugin,  // Temporarily disabled
            // ability_display::AbilityDisplayPlugin,  // Temporarily disabled
            // Temporarily disabled for integration testing
            // shop_menu::ShopMenuPlugin,
            // talent_menu::TalentMenuPlugin,
            // achievement_display::AchievementDisplayPlugin,
            
            // Advanced menu system (replaces pause_menu)
            menus::main_game_menu::MainGameMenuPlugin,
            
            // Enhanced HUD
            components::game_hud::GameHudPlugin,
        ));
    }
}
