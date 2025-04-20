use bevy::prelude::*;

use crate::resources::score::Score;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("score: {}", score.value.to_string());
    }
}
