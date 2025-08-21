pub mod main_menu;
pub mod pause_menu;
pub mod hud;
pub mod health_bars;
//pub mod minimap;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hud::HUDPlugin,
            main_menu::MainMenuPlugin,
            pause_menu::PauseMenuPlugin,
            health_bars::HealthBarPlugin,
//            minimap::MinimapPlugin,
        ));
    }
}
