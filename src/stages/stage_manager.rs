use bevy::prelude::*;
use crate::entities::enemy::{SpawnEnemy, EnemyType};
use crate::entities::fruit_spawner::Fruit;

pub struct StageManagerPlugin;

impl Plugin for StageManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentStage>()
            .init_resource::<StageProgress>()
            .add_event::<StageComplete>()
            .add_event::<NextStage>()
            .add_systems(Update, (
                check_stage_completion,
                handle_stage_transition,
            ));
    }
}

#[derive(Resource)]
pub struct CurrentStage {
    pub stage_number: u32,
    pub enemies_spawned: u32,
    pub enemies_remaining: u32,
    pub fruits_collected: u32,
}

impl Default for CurrentStage {
    fn default() -> Self {
        Self {
            stage_number: 1,
            enemies_spawned: 0,
            enemies_remaining: 0,
            fruits_collected: 0,
        }
    }
}

#[derive(Resource)]
pub struct StageProgress {
    pub total_stages_completed: u32,
    pub total_enemies_defeated: u32,
    pub total_fruits_collected: u32,
}

impl Default for StageProgress {
    fn default() -> Self {
        Self {
            total_stages_completed: 0,
            total_enemies_defeated: 0,
            total_fruits_collected: 0,
        }
    }
}

#[derive(Event)]
pub struct StageComplete {
    pub stage_number: u32,
}

#[derive(Event)]
pub struct NextStage;

fn check_stage_completion(
    current_stage: Res<CurrentStage>,
    enemies: Query<Entity, With<crate::entities::enemy::Enemy>>,
    mut complete_events: EventWriter<StageComplete>,
) {
    if current_stage.enemies_spawned > 0 && enemies.iter().count() == 0 {
        complete_events.send(StageComplete {
            stage_number: current_stage.stage_number,
        });
    }
}

fn handle_stage_transition(
    mut current_stage: ResMut<CurrentStage>,
    mut stage_progress: ResMut<StageProgress>,
    mut complete_events: EventReader<StageComplete>,
    mut next_events: EventReader<NextStage>,
    mut spawn_events: EventWriter<SpawnEnemy>,
    mut commands: Commands,
    fruits: Query<Entity, With<Fruit>>,
) {
    for event in complete_events.read() {
        stage_progress.total_stages_completed += 1;
        println!("Stage {} complete!", event.stage_number);
    }
    
    for _ in next_events.read() {
        current_stage.stage_number += 1;
        current_stage.enemies_spawned = 0;
        current_stage.enemies_remaining = 0;
        current_stage.fruits_collected = 0;
        
        for entity in fruits.iter() {
            commands.entity(entity).despawn();
        }
        
        let enemies_to_spawn = calculate_enemies_for_stage(current_stage.stage_number);
        
        for i in 0..enemies_to_spawn {
            let enemy_type = match i % 3 {
                0 => EnemyType::Goblin,
                1 => EnemyType::Skeleton,
                _ => EnemyType::Orc,
            };
            
            let angle = (i as f32 / enemies_to_spawn as f32) * std::f32::consts::TAU;
            let radius = 200.0 + (i as f32 * 20.0);
            let position = Vec3::new(
                angle.cos() * radius,
                angle.sin() * radius,
                3.0,
            );
            
            spawn_events.send(SpawnEnemy {
                position,
                kind: enemy_type,
            });
        }
        
        current_stage.enemies_spawned = enemies_to_spawn;
        current_stage.enemies_remaining = enemies_to_spawn;
    }
}

fn calculate_enemies_for_stage(stage_number: u32) -> u32 {
    3 + (stage_number - 1) * 2
}
