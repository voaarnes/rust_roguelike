use bevy::prelude::*;
use super::player::{Player, PlayerStats, PlayerVisuals, spawn_player_with_parts, update_player_movement_with_stats, update_player_body_parts};

pub struct UpdatedPlayerPlugin;

impl Plugin for UpdatedPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_updated_player)
            .add_systems(Update, (
                update_player_movement_with_stats,
                update_player_body_parts,
            ));
    }
}

fn spawn_updated_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_player_with_parts(&mut commands, &asset_server, &mut layouts, Vec3::new(0.0, 0.0, 10.0));
}
