pub mod powerup_display;
pub mod hud;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            powerup_display::PowerUpDisplayPlugin,
            hud::HudPlugin,
        ));
    }
}
