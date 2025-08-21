use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub animations: std::collections::HashMap<String, AnimationClip>,
    pub current: String,
    pub timer: Timer,
    pub frame: usize,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            animations: std::collections::HashMap::new(),
            current: "idle".to_string(),
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame: 0,
        }
    }
    
    pub fn add_animation(&mut self, name: &str, clip: AnimationClip) {
        self.animations.insert(name.to_string(), clip);
    }
    
    pub fn play(&mut self, name: &str) {
        if self.animations.contains_key(name) && self.current != name {
            self.current = name.to_string();
            self.frame = self.animations[name].start;
            self.timer = Timer::from_seconds(
                self.animations[name].frame_time,
                if self.animations[name].looping {
                    TimerMode::Repeating
                } else {
                    TimerMode::Once
                },
            );
        }
    }
    
    pub fn is_finished(&self) -> bool {
        if let Some(clip) = self.animations.get(&self.current) {
            !clip.looping && self.frame >= clip.end
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct AnimationClip {
    pub start: usize,
    pub end: usize,
    pub frame_time: f32,
    pub looping: bool,
}

impl AnimationClip {
    pub fn new(start: usize, end: usize, frame_time: f32, looping: bool) -> Self {
        Self { start, end, frame_time, looping }
    }
}

fn animate_sprites(
    mut query: Query<(&mut AnimationController, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut controller, mut sprite) in query.iter_mut() {
        controller.timer.tick(time.delta());
        
        if controller.timer.just_finished() {
            if let Some(clip) = controller.animations.get(&controller.current) {
                controller.frame += 1;
                if controller.frame > clip.end {
                    if clip.looping {
                        controller.frame = clip.start;
                    } else {
                        controller.frame = clip.end;
                    }
                }
                
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = controller.frame;
                }
            }
        }
    }
}
