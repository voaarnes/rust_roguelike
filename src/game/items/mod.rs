use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemAssets>();
    }
}

#[derive(Resource, Default)]
pub struct ItemAssets {
    pub textures: Vec<Handle<Image>>,
}
