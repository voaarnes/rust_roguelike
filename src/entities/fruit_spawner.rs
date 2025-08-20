use bevy::prelude::*;
use super::powerup::{PowerUpType, CollectPowerUp};
use crate::tilemap::tilemap::MapSizePx;
use rand::Rng;

pub struct FruitSpawnerPlugin;

impl Plugin for FruitSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FruitSpawnTimer>()
            .add_systems(Startup, spawn_initial_fruits)
            .add_systems(Update, (
                spawn_fruits_periodically,
                handle_fruit_collection,
                animate_fruits,
            ));
    }
}

#[derive(Resource)]
struct FruitSpawnTimer {
    timer: Timer,
    min_fruits: usize,
    max_fruits: usize,
}

impl Default for FruitSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
            min_fruits: 3,
            max_fruits: 8,
        }
    }
}

#[derive(Component)]
pub struct Fruit {
    pub power_type: PowerUpType,
    pub bob_offset: f32,
    pub bob_speed: f32,
    pub collection_radius: f32,
}

fn spawn_initial_fruits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_size: Option<Res<MapSizePx>>,
) {
    let positions = if let Some(map) = map_size {
        vec![
            Vec3::new(-map.w * 0.3, map.h * 0.2, 5.0),
            Vec3::new(map.w * 0.3, -map.h * 0.2, 5.0),
            Vec3::new(0.0, map.h * 0.3, 5.0),
            Vec3::new(-map.w * 0.2, -map.h * 0.3, 5.0),
            Vec3::new(map.w * 0.2, 0.0, 5.0),
        ]
    } else {
        vec![
            Vec3::new(-200.0, 100.0, 5.0),
            Vec3::new(200.0, -100.0, 5.0),
            Vec3::new(0.0, 150.0, 5.0),
            Vec3::new(-100.0, -150.0, 5.0),
            Vec3::new(100.0, 0.0, 5.0),
        ]
    };
    
    let fruit_types = [
        PowerUpType::Strawberry,
        PowerUpType::Pear,
        PowerUpType::Mango,
        PowerUpType::Apple,
        PowerUpType::Orange,
    ];
    
    for (pos, fruit_type) in positions.iter().zip(fruit_types.iter()) {
        spawn_fruit(&mut commands, &asset_server, *pos, *fruit_type);
    }
}

fn spawn_fruits_periodically(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<FruitSpawnTimer>,
    fruits: Query<Entity, With<Fruit>>,
    map_size: Option<Res<MapSizePx>>,
) {
    spawn_timer.timer.tick(time.delta());
    
    if spawn_timer.timer.just_finished() {
        let current_fruits = fruits.iter().count();
        
        if current_fruits < spawn_timer.min_fruits {
            let mut rng = rand::thread_rng();
            let fruits_to_spawn = rng.gen_range(1..=3);
            
            for _ in 0..fruits_to_spawn {
                let fruit_type = random_fruit_type();
                let position = if let Some(map) = map_size.as_ref() {
                    Vec3::new(
                        rng.gen_range(-map.w * 0.4..map.w * 0.4),
                        rng.gen_range(-map.h * 0.4..map.h * 0.4),
                        5.0,
                    )
                } else {
                    Vec3::new(
                        rng.gen_range(-300.0..300.0),
                        rng.gen_range(-200.0..200.0),
                        5.0,
                    )
                };
                
                spawn_fruit(&mut commands, &asset_server, position, fruit_type);
            }
        }
    }
}

fn spawn_fruit(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    fruit_type: PowerUpType,
) {
    let mut rng = rand::thread_rng();
    
    let texture = asset_server.load("sprites/fruits.png");
    
    let sprite_index = match fruit_type {
        PowerUpType::Strawberry => 0,
        PowerUpType::Pear => 1,
        PowerUpType::Mango => 2,
        PowerUpType::Apple => 3,
        PowerUpType::Orange => 4,
        PowerUpType::Grape => 5,
        PowerUpType::Banana => 6,
        PowerUpType::Cherry => 7,
    };
    
    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_translation(position),
        Fruit {
            power_type: fruit_type,
            bob_offset: rng.gen_range(0.0..std::f32::consts::TAU),
            bob_speed: rng.gen_range(2.0..4.0),
            collection_radius: 32.0,
        },
    ));
}

fn handle_fruit_collection(
    mut commands: Commands,
    fruits: Query<(Entity, &Transform, &Fruit)>,
    player: Query<&Transform, With<crate::entities::player::Player>>,
    mut collect_events: EventWriter<CollectPowerUp>,
    audio: Res<crate::audio::AudioManager>,
) {
    if let Ok(player_transform) = player.get_single() {
        for (entity, fruit_transform, fruit) in fruits.iter() {
            let distance = player_transform.translation.distance(fruit_transform.translation);
            
            if distance < fruit.collection_radius {
                collect_events.send(CollectPowerUp {
                    power_type: fruit.power_type,
                });
                
                audio.play_sound(&mut commands, "collect", 0.5);
                
                commands.entity(entity).despawn();
            }
        }
    }
}

fn animate_fruits(
    mut fruits: Query<(&mut Transform, &Fruit)>,
    time: Res<Time>,
) {
    for (mut transform, fruit) in fruits.iter_mut() {
        let bob = (time.elapsed_secs() * fruit.bob_speed + fruit.bob_offset).sin() * 5.0;
        transform.translation.y += bob * time.delta_secs();
        transform.rotate_z(2.0 * time.delta_secs());
    }
}

fn random_fruit_type() -> PowerUpType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..8) {
        0 => PowerUpType::Strawberry,
        1 => PowerUpType::Pear,
        2 => PowerUpType::Mango,
        3 => PowerUpType::Apple,
        4 => PowerUpType::Orange,
        5 => PowerUpType::Grape,
        6 => PowerUpType::Banana,
        _ => PowerUpType::Cherry,
    }
}
