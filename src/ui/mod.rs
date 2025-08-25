pub mod main_menu;
pub mod hud;
pub mod health_bars;
pub mod minimap;
pub mod powerup_display;
pub mod ability_display;

// New advanced UI modules
pub mod components;
pub mod menus;
pub mod overlays;

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
            powerup_display::PowerUpDisplayPlugin,
            ability_display::AbilityDisplayPlugin,
            
            // Advanced menu system (replaces pause_menu)
            menus::main_game_menu::MainGameMenuPlugin,
            
            // Enhanced HUD
            components::game_hud::GameHudPlugin,
        ));
    }
}
