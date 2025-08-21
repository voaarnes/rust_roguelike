use bevy::prelude::*;
use rand::Rng;
use crate::game::enemy::{SpawnEnemyEvent, SpawnBossEvent, EnemyType};
use crate::core::config::GameConfig;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveManager>()
            .add_systems(Update, (
                spawn_wave_system,
                update_difficulty,
            ));
    }
}

#[derive(Resource)]
pub struct WaveManager {
    pub current_wave: u32,
    pub enemies_remaining: u32,
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub difficulty_multiplier: f32,
    pub boss_spawned: bool,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_remaining: 0,
            wave_timer: Timer::from_seconds(30.0, TimerMode::Once),
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            difficulty_multiplier: 1.0,
            boss_spawned: false,
        }
    }
}

fn spawn_wave_system(
    mut wave_manager: ResMut<WaveManager>,
    mut spawn_events: EventWriter<SpawnEnemyEvent>,
    mut boss_events: EventWriter<SpawnBossEvent>,
    player_q: Query<&Transform, With<crate::game::player::Player>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    wave_manager.wave_timer.tick(time.delta());
    wave_manager.spawn_timer.tick(time.delta());
    
    // Start new wave
    if wave_manager.wave_timer.finished() && wave_manager.enemies_remaining == 0 {
        wave_manager.current_wave += 1;
        wave_manager.enemies_remaining = calculate_wave_enemies(wave_manager.current_wave);
        wave_manager.wave_timer.reset();
        
        // Boss wave every 5 waves
        if wave_manager.current_wave % 5 == 0 && !wave_manager.boss_spawned {
            wave_manager.boss_spawned = true;
            if let Ok(player_tf) = player_q.get_single() {
                boss_events.write(SpawnBossEvent {
                    position: player_tf.translation + Vec3::new(300.0, 0.0, 3.0),
                    boss_type: choose_boss_type(wave_manager.current_wave),
                });
            }
        }
    }
    
    // Spawn enemies
    if wave_manager.spawn_timer.just_finished() && wave_manager.enemies_remaining > 0 {
        if let Ok(player_tf) = player_q.get_single() {
            let mut rng = rand::thread_rng();
            let spawn_count = (3.0 * wave_manager.difficulty_multiplier) as u32;
            
            for _ in 0..spawn_count.min(wave_manager.enemies_remaining) {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(200.0..400.0);
                let spawn_pos = Vec3::new(
                    player_tf.translation.x + angle.cos() * distance,
                    player_tf.translation.y + angle.sin() * distance,
                    3.0,
                );
                
                spawn_events.write(SpawnEnemyEvent {
                    position: spawn_pos,
                    enemy_type: choose_enemy_type(wave_manager.current_wave),
                });
                
                wave_manager.enemies_remaining -= 1;
            }
        }
    }
}

fn update_difficulty(
    mut wave_manager: ResMut<WaveManager>,
    time: Res<Time>,
) {
    // Increase difficulty over time
    wave_manager.difficulty_multiplier = 1.0 + (time.elapsed_secs() / 60.0) * 0.2;
}

fn calculate_wave_enemies(wave: u32) -> u32 {
    10 + wave * 5
}

fn choose_enemy_type(wave: u32) -> EnemyType {
    let mut rng = rand::thread_rng();
    if wave < 3 {
        EnemyType::Goblin
    } else if wave < 6 {
        match rng.gen_range(0..2) {
            0 => EnemyType::Goblin,
            _ => EnemyType::Skeleton,
        }
    } else if wave < 10 {
        match rng.gen_range(0..3) {
            0 => EnemyType::Goblin,
            1 => EnemyType::Skeleton,
            _ => EnemyType::Orc,
        }
    } else {
        match rng.gen_range(0..5) {
            0 => EnemyType::Goblin,
            1 => EnemyType::Skeleton,
            2 => EnemyType::Orc,
            3 => EnemyType::DarkKnight,
            _ => EnemyType::Necromancer,
        }
    }
}

fn choose_boss_type(wave: u32) -> EnemyType {
    if wave < 10 {
        EnemyType::GoblinKing
    } else if wave < 20 {
        EnemyType::LichLord
    } else {
        EnemyType::DragonKnight
    }
}
