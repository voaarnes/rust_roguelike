pub mod main_menu;
pub mod pause_menu;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(pause_menu::PauseMenuPlugin);
    }
}
