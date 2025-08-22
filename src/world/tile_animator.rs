use bevy::prelude::*;
use super::tilemap::AnimatedTile;

pub fn animate_tiles(
    time: Res<Time>,
    mut q: Query<(&mut AnimatedTile, &mut Sprite)>,
) {
    for (mut anim, mut sprite) in &mut q {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = anim.frames[anim.current_frame];
            }
        }
    }
}
