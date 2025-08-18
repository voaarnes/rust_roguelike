use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Clone)]
pub struct SpriteSheetAnimation {
    pub animations: HashMap<String, AnimationClip>,
    pub current_animation: String,
    pub timer: Timer,
    pub current_frame: usize,
    pub is_looping: bool,
    pub is_playing: bool,
}

#[derive(Clone)]
pub struct AnimationClip {
    pub start_index: usize,
    pub end_index: usize,
    pub frame_duration: f32,
}

impl SpriteSheetAnimation {
    pub fn new(frame_duration: f32) -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: String::from("idle"),
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            current_frame: 0,
            is_looping: true,
            is_playing: true,
        }
    }

    pub fn add_animation(&mut self, name: String, clip: AnimationClip) {
        self.animations.insert(name, clip);
    }

    pub fn play(&mut self, animation_name: &str, looping: bool) {
        if self.animations.contains_key(animation_name) {
            self.current_animation = animation_name.to_string();
            self.is_looping = looping;
            self.is_playing = true;
            self.current_frame = self.animations[animation_name].start_index;
            self.timer = Timer::from_seconds(
                self.animations[animation_name].frame_duration,
                if looping { TimerMode::Repeating } else { TimerMode::Once },
            );
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }
}
