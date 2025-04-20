use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
