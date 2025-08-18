use super::sprite_sheet::SpriteSheetAnimation;
use bevy::prelude::*;

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteSheetAnimation, &mut Sprite)>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        if !animation.is_playing {
            continue;
        }

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(clip) = animation.animations.get(&animation.current_animation) {
                animation.current_frame += 1;

                if animation.current_frame > clip.end_index {
                    if animation.is_looping {
                        animation.current_frame = clip.start_index;
                    } else {
                        animation.current_frame = clip.end_index;
                        animation.is_playing = false;
                    }
                }

                // Update the sprite's texture atlas index
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = animation.current_frame;
                }
            }
        }
    }
}
