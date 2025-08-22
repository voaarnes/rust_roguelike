use bevy::prelude::*;
use super::tilemap::AnimatedField;

pub fn animate_tiles(
    mut query: Query<(&mut AnimatedField, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animated, mut sprite) in query.iter_mut() {
        animated.timer.tick(time.delta());
        
        if animated.timer.just_finished() {
            animated.current_frame = (animated.current_frame + 1) % animated.frames.len();
            
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animated.frames[animated.current_frame];
            }
        }
    }
}
