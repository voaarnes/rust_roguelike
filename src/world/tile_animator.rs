use bevy::prelude::*;
use super::tilemap::AnimatedTile;

pub fn animate_tiles(
    time: Res<Time>,
    mut query: Query<(&mut AnimatedTile, &mut TextureAtlasSprite)>, // <-- use TextureAtlasSprite
) {
    for (mut animated_tile, mut sprite) in query.iter_mut() {
        animated_tile.timer.tick(time.delta());

        if animated_tile.timer.just_finished() {
            animated_tile.current_frame =
                (animated_tile.current_frame + 1) % animated_tile.frames.len();

            sprite.index = animated_tile.frames[animated_tile.current_frame];
        }
    }
}
