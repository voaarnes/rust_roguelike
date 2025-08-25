pub mod main_menu;
pub mod pause_menu;
pub mod hud;
pub mod health_bars;
pub mod minimap;
pub mod powerup_display;
pub mod ability_display;

// New advanced UI modules - temporarily disabled for Bevy 0.16 compatibility
// pub mod components;
// pub mod menus;
// pub mod overlays;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Existing UI plugins
            hud::HUDPlugin,
            main_menu::MainMenuPlugin,
            pause_menu::PauseMenuPlugin,
            health_bars::HealthBarPlugin,
            minimap::MinimapPlugin,
            powerup_display::PowerUpDisplayPlugin,
            ability_display::AbilityDisplayPlugin,
            
            // New advanced UI plugins - temporarily disabled for Bevy 0.16 compatibility
            // components::game_hud::GameHudPlugin,
            // menus::main_game_menu::MainGameMenuPlugin,
        ));
    }
}
