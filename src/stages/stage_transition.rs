use bevy::prelude::*;
use super::stage_manager::{StageComplete, NextStage};

pub struct StageTransitionPlugin;

impl Plugin for StageTransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TransitionTimer>()
            .add_systems(Update, handle_transition);
    }
}

#[derive(Resource)]
struct TransitionTimer {
    timer: Timer,
    active: bool,
}

impl Default for TransitionTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            active: false,
        }
    }
}

fn handle_transition(
    mut transition_timer: ResMut<TransitionTimer>,
    mut complete_events: EventReader<StageComplete>,
    mut next_events: EventWriter<NextStage>,
    time: Res<Time>,
) {
    for _ in complete_events.read() {
        transition_timer.active = true;
        transition_timer.timer.reset();
    }
    
    if transition_timer.active {
        transition_timer.timer.tick(time.delta());
        
        if transition_timer.timer.just_finished() {
            next_events.send(NextStage);
            transition_timer.active = false;
        }
    }
}
