
#!/bin/bash

# Fix the borrowing issue in the animation system
cat > src/game/animation/mod.rs << 'EOF'
use bevy::prelude::*;
use std::collections::HashMap;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animations);
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub animations: HashMap<String, AnimationClip>,
    pub current: String,
    pub timer: Timer,
    pub frame_index: usize,
}

#[derive(Clone)]
pub struct AnimationClip {
    pub start_frame: usize,
    pub end_frame: usize,
    pub frame_duration: f32,
    pub looping: bool,
}

impl AnimationClip {
    pub fn new(start_frame: usize, end_frame: usize, frame_duration: f32, looping: bool) -> Self {
        Self {
            start_frame,
            end_frame,
            frame_duration,
            looping,
        }
    }
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current: String::new(),
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
        }
    }
    
    pub fn add_animation(&mut self, name: &str, clip: AnimationClip) {
        self.animations.insert(name.to_string(), clip);
    }
    
    pub fn play(&mut self, name: &str) {
        if self.current != name {
            self.current = name.to_string();
            if let Some(clip) = self.animations.get(name) {
                self.frame_index = clip.start_frame;
                self.timer = Timer::from_seconds(clip.frame_duration, TimerMode::Repeating);
                self.timer.reset();
            }
        }
    }
}

fn update_animations(
    mut query: Query<(&mut AnimationController, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut controller, mut sprite) in query.iter_mut() {
        // Get the clip data first to avoid borrowing conflicts
        let current_anim = controller.current.clone();
        let clip_data = if let Some(clip) = controller.animations.get(&current_anim) {
            Some((clip.start_frame, clip.end_frame, clip.looping))
        } else {
            None
        };
        
        if let Some((start_frame, end_frame, looping)) = clip_data {
            controller.timer.tick(time.delta());
            
            if controller.timer.just_finished() {
                controller.frame_index += 1;
                
                if controller.frame_index > end_frame {
                    if looping {
                        controller.frame_index = start_frame;
                    } else {
                        controller.frame_index = end_frame;
                    }
                }
                
                // Update sprite atlas index if it has a texture atlas
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = controller.frame_index;
                }
            }
        }
    }
}
EOF

echo "Fixed animation system borrowing issue!"
