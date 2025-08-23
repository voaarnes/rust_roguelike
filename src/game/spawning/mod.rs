use bevy::prelude::*;
use rand::Rng;
use crate::game::enemy::{SpawnEnemyEvent, SpawnBossEvent, EnemyType};
use crate::core::state::GameState;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveManager>()
            .init_resource::<EnemySpawnTimer>()
            .add_systems(Update, (
                spawn_wave_system.run_if(in_state(GameState::Playing)),
                update_difficulty.run_if(in_state(GameState::Playing)),
                spawn_collectibles.run_if(in_state(GameState::Playing)),
            ));
    }
}

#[derive(Resource)]
pub struct WaveManager {
    pub current_wave: u32,
    pub enemies_spawned: u32,
    pub enemies_alive: u32,
    pub wave_timer: Timer,
    pub difficulty_multiplier: f32,
    pub boss_spawned: bool,
    pub wave_complete: bool,
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_spawned: 0,
            enemies_alive: 0,
            wave_timer: Timer::from_seconds(30.0, TimerMode::Once),
            difficulty_multiplier: 1.0,
            boss_spawned: false,
            wave_complete: false,
        }
    }
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

fn spawn_wave_system(
    mut wave_manager: ResMut<WaveManager>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut spawn_events: EventWriter<SpawnEnemyEvent>,
    mut boss_events: EventWriter<SpawnBossEvent>,
    player_q: Query<&Transform, With<crate::game::player::Player>>,
    enemy_q: Query<&Transform, With<crate::game::enemy::Enemy>>,
    time: Res<Time>,
) {
    wave_manager.wave_timer.tick(time.delta());
    spawn_timer.0.tick(time.delta());
    
    // Update alive enemy count
    wave_manager.enemies_alive = enemy_q.iter().count() as u32;
    
    // Check if wave is complete
    if wave_manager.enemies_spawned >= calculate_wave_enemies(wave_manager.current_wave) 
        && wave_manager.enemies_alive == 0 
        && !wave_manager.boss_spawned {
        wave_manager.wave_complete = true;
    }
    
    // Start new wave
    if wave_manager.wave_complete || (wave_manager.current_wave == 1 && wave_manager.enemies_spawned == 0) {
        wave_manager.current_wave += 1;
        wave_manager.enemies_spawned = 0;
        wave_manager.wave_complete = false;
        wave_manager.boss_spawned = false;
        wave_manager.wave_timer.reset();
        
        println!("Starting wave {}", wave_manager.current_wave);
        
        // Boss wave every 5 waves
        if wave_manager.current_wave % 5 == 0 {
            wave_manager.boss_spawned = true;
            if let Ok(player_tf) = player_q.single() {
                boss_events.write(SpawnBossEvent {
                    position: player_tf.translation + Vec3::new(300.0, 0.0, 3.0),
                    boss_type: choose_boss_type(wave_manager.current_wave),
                });
                println!("Spawning boss!");
            }
        }
    }
    
    // Spawn regular enemies
    if spawn_timer.0.just_finished() && 
       wave_manager.enemies_spawned < calculate_wave_enemies(wave_manager.current_wave) {
        if let Ok(player_tf) = player_q.single() {
            let mut rng = rand::thread_rng();
            let spawn_count = 1u32.max((3.0 * wave_manager.difficulty_multiplier) as u32);
            
            for _ in 0..spawn_count {
                if wave_manager.enemies_spawned >= calculate_wave_enemies(wave_manager.current_wave) {
                    break;
                }
                
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
                
                wave_manager.enemies_spawned += 1;
                println!("Spawned enemy {} of {}", wave_manager.enemies_spawned, calculate_wave_enemies(wave_manager.current_wave));
            }
        }
    }
}


fn spawn_collectibles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_q: Query<&Transform, With<crate::game::player::Player>>,
    collectible_q: Query<&Transform, With<crate::game::collectible::Collectible>>,
    _time: Res<Time>,
) {
    if collectible_q.iter().count() < 10 {
        if let Ok(player_tf) = player_q.single() {
            if rand::random::<f32>() < 0.02 {
                let mut rng = rand::thread_rng();
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(100.0..300.0);
                let spawn_pos = Vec3::new(
                    player_tf.translation.x + angle.cos() * distance,
                    player_tf.translation.y + angle.sin() * distance,
                    2.0,
                );

                // Texture atlas has 8 frames (0..7)
                let texture = asset_server.load("sprites/meyveler.png");
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(32, 32),
                    7, 1, 
                    None, None,
                );
                let layout_handle = layouts.add(layout);

                let fruit_type = rng.gen_range(0..7);

                println!("fruit spawn {}", fruit_type);

                let scale = if fruit_type == 6 { 1.0 } else { 2.0 }; // double size except #7

                commands.spawn((
                    crate::game::collectible::Collectible {
                        collectible_type: crate::game::collectible::CollectibleType::Fruit(fruit_type),
                        value: 1,
                    },
                    Sprite {
                        image: texture,
                        texture_atlas: Some(TextureAtlas {
                            layout: layout_handle,
                            index: fruit_type as usize,
                        }),
                        ..default()
                    },

                    // Scale only the visuals; collider below stays the same size.
                    Transform::from_translation(spawn_pos).with_scale(Vec3::splat(scale)),
                    crate::game::movement::Collider { size: Vec2::splat(24.0) },
                ));
            }
        }
    }
}

fn update_difficulty(
    mut wave_manager: ResMut<WaveManager>,
    _time: Res<Time>,
) {
    // Increase difficulty over time
    wave_manager.difficulty_multiplier = 1.0 + (wave_manager.current_wave as f32 / 10.0) * 0.2;
}

fn calculate_wave_enemies(wave: u32) -> u32 {
    5 + wave * 2 // Start smaller for testing
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
