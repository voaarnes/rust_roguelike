pub mod sprite_sheet;
pub mod animator;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animator::animate_sprites);
    }
}
