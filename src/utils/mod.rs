use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, _app: &mut App) {
        // Utility systems
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn random_in_circle(radius: f32) -> Vec2 {
    let angle = rand::random::<f32>() * std::f32::consts::TAU;
    let r = rand::random::<f32>() * radius;
    Vec2::new(angle.cos() * r, angle.sin() * r)
}
