use super::tilemap::AnimatedTile;
use bevy::prelude::*;

pub fn animate_tiles(
    time: Res<Time>,
    mut query: Query<(&mut AnimatedTile, &mut Sprite)>,
) {
    for (mut animated_tile, mut sprite) in query.iter_mut() {
        animated_tile.timer.tick(time.delta());
        
        if animated_tile.timer.just_finished() {
            // Advance to next frame
            animated_tile.current_frame = 
                (animated_tile.current_frame + 1) % animated_tile.frames.len();
            
            // Update the sprite's texture atlas index
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animated_tile.frames[animated_tile.current_frame];
            }
        }
    }
}
