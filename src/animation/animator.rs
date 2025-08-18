use super::sprite_sheet::SpriteSheetAnimation;
use bevy::prelude::*;

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteSheetAnimation, &mut Sprite)>,
    _texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>, // underscore to silence "unused"
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        if !animation.is_playing {
            continue;
        }

        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }

        // Take the data we need from the clip, then drop the borrow before mutating `animation`.
        let (start_index, end_index) = match animation.animations.get(&animation.current_animation) {
            Some(clip) => (clip.start_index, clip.end_index),
            None => continue,
        };

        // Advance frame
        animation.current_frame += 1;

        // Wrap or stop
        if animation.current_frame > end_index {
            if animation.is_looping {
                animation.current_frame = start_index;
            } else {
                animation.current_frame = end_index;
                animation.is_playing = false;
            }
        }

        // Push the frame to the sprite's atlas index
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = animation.current_frame;
        }
    }
}
