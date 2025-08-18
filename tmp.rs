// src/animation/mod.rs
pub mod sprite_sheet;
pub mod animator;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animator::animate_sprites);
    }
}

// src/animation/sprite_sheet.rs
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

// src/animation/animator.rs
use super::sprite_sheet::SpriteSheetAnimation;
use bevy::prelude::*;

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteSheetAnimation, &mut Sprite)>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        if !animation.is_playing {
            continue;
        }

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(clip) = animation.animations.get(&animation.current_animation) {
                animation.current_frame += 1;

                if animation.current_frame > clip.end_index {
                    if animation.is_looping {
                        animation.current_frame = clip.start_index;
                    } else {
                        animation.current_frame = clip.end_index;
                        animation.is_playing = false;
                    }
                }

                // Update the sprite's texture atlas index
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = animation.current_frame;
                }
            }
        }
    }
}

// src/audio/mod.rs
use bevy::prelude::*;
use std::collections::HashMap;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioManager>()
            .add_systems(Startup, setup_audio);
    }
}

#[derive(Resource)]
pub struct AudioManager {
    pub sounds: HashMap<String, Handle<AudioSource>>,
    pub music: HashMap<String, Handle<AudioSource>>,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            sounds: HashMap::new(),
            music: HashMap::new(),
        }
    }
}

impl AudioManager {
    pub fn play_sound(
        &self,
        commands: &mut Commands,
        sound_name: &str,
        volume: f32,
    ) {
        if let Some(sound) = self.sounds.get(sound_name) {
            commands.spawn((
                AudioPlayer::new(sound.clone()),
                PlaybackSettings::ONCE.with_volume(volume),
            ));
        }
    }

    pub fn play_music(
        &self,
        commands: &mut Commands,
        music_name: &str,
        volume: f32,
    ) {
        if let Some(music) = self.music.get(music_name) {
            commands.spawn((
                AudioPlayer::new(music.clone()),
                PlaybackSettings::LOOP.with_volume(volume),
            ));
        }
    }
}

fn setup_audio(
    mut audio_manager: ResMut<AudioManager>,
    asset_server: Res<AssetServer>,
) {
    // Load sound effects
    audio_manager.sounds.insert(
        "hit".to_string(),
        asset_server.load("audio/audio_001.ogg"),
    );
    audio_manager.sounds.insert(
        "collect".to_string(),
        asset_server.load("audio/audio_002.ogg"),
    );
    
    // Load music tracks
    // audio_manager.music.insert(
    //     "main_theme".to_string(),
    //     asset_server.load("audio/main_theme.ogg"),
    // );
}

// src/tilemap/mod.rs
pub mod tilemap;
pub mod tile_loader;

use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tile_loader::load_test_level);
    }
}

// src/tilemap/tilemap.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
    pub tile_index: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Chest,
    Spike,
    Water,
}

#[derive(Resource)]
pub struct TilemapConfig {
    pub tile_size: f32,
    pub tileset_columns: usize,
    pub tileset_rows: usize,
}

impl Default for TilemapConfig {
    fn default() -> Self {
        Self {
            tile_size: 32.0,
            tileset_columns: 16,
            tileset_rows: 16,
        }
    }
}

pub struct Tilemap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Option<TileType>>>,
}

impl Tilemap {
    pub fn from_string(map_string: &str) -> Self {
        let lines: Vec<&str> = map_string.lines().collect();
        let height = lines.len();
        let width = lines.get(0).map_or(0, |line| line.len());
        
        let mut tiles = vec![vec![None; width]; height];
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                tiles[y][x] = match ch {
                    '#' => Some(TileType::Wall),
                    '.' => Some(TileType::Floor),
                    'D' => Some(TileType::Door),
                    'C' => Some(TileType::Chest),
                    '^' => Some(TileType::Spike),
                    '~' => Some(TileType::Water),
                    _ => None,
                };
            }
        }
        
        Self { width, height, tiles }
    }
    
    pub fn from_indices(map_data: Vec<Vec<usize>>) -> Self {
        let height = map_data.len();
        let width = map_data.get(0).map_or(0, |row| row.len());
        
        let mut tiles = vec![vec![None; width]; height];
        
        for (y, row) in map_data.iter().enumerate() {
            for (x, &index) in row.iter().enumerate() {
                tiles[y][x] = match index {
                    0 => None,
                    1..=16 => Some(TileType::Floor),
                    17..=32 => Some(TileType::Wall),
                    33..=36 => Some(TileType::Door),
                    37..=40 => Some(TileType::Chest),
                    41..=44 => Some(TileType::Spike),
                    45..=48 => Some(TileType::Water),
                    _ => None,
                };
            }
        }
        
        Self { width, height, tiles }
    }
}

// src/tilemap/tile_loader.rs
use super::tilemap::{Tile, TileType, Tilemap, TilemapConfig};
use bevy::prelude::*;

pub fn load_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Example level data - you can load this from a file
    let level_data = "
########################################
#......................................#
#.....###...###...###..................#
#.....#.....#.#...#.#..................#
#.....###...#.#...###..................#
#.....#.....#.#...#.#..................#
#.....#.....###...#.#..................#
#......................................#
#......................................#
#...........^^^^.......................#
#......................................#
#......................................#
#.....CCCC.............................#
#......................................#
#......................................#
#.....................~~~~~~~~~~~~~~~~~#
#.....................~~~~~~~~~~~~~~~~~#
#.....................~~~~~~~~~~~~~~~~~#
########################################
";

    let tilemap = Tilemap::from_string(level_data);
    let config = TilemapConfig::default();
    
    // Load the tileset texture
    let tileset_handle: Handle<Image> = asset_server.load("sprites/tileset.png");
    
    // Create texture atlas layout
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.tile_size as u32, config.tile_size as u32),
        config.tileset_columns as u32,
        config.tileset_rows as u32,
        None,
        None,
    );
    
    // Spawn tiles
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            if let Some(tile_type) = tilemap.tiles[y][x] {
                let tile_index = get_tile_index(tile_type);
                let world_pos = Vec3::new(
                    x as f32 * config.tile_size,
                    (tilemap.height - y - 1) as f32 * config.tile_size,
                    0.0,
                );
                
                commands.spawn((
                    Sprite {
                        image: tileset_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: Handle::weak_from_u128(0), // You'll need to properly handle this
                            index: tile_index,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                    Tile {
                        tile_type,
                        walkable: is_walkable(tile_type),
                        tile_index,
                    },
                ));
            }
        }
    }
}

fn get_tile_index(tile_type: TileType) -> usize {
    match tile_type {
        TileType::Floor => 1,
        TileType::Wall => 17,
        TileType::Door => 33,
        TileType::Chest => 37,
        TileType::Spike => 41,
        TileType::Water => 45,
    }
}

fn is_walkable(tile_type: TileType) -> bool {
    match tile_type {
        TileType::Floor | TileType::Door => true,
        TileType::Wall | TileType::Chest | TileType::Spike | TileType::Water => false,
    }
}

// src/entities/mod.rs
pub mod player;
pub mod enemy;
pub mod collectible;

use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            collectible::CollectiblePlugin,
        ));
    }
}

// src/entities/player.rs
use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 500.0,
            health: 100,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/player_sheet.png");
    
    // Create texture atlas layout for player sprite sheet
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4,  // columns
        4,  // rows
        None,
        None,
    );
    
    let mut animation = SpriteSheetAnimation::new(0.1);
    
    // Define animations
    animation.add_animation(
        "idle".to_string(),
        AnimationClip {
            start_index: 0,
            end_index: 3,
            frame_duration: 0.2,
        },
    );
    
    animation.add_animation(
        "walk".to_string(),
        AnimationClip {
            start_index: 4,
            end_index: 7,
            frame_duration: 0.1,
        },
    );
    
    animation.add_animation(
        "attack".to_string(),
        AnimationClip {
            start_index: 8,
            end_index: 11,
            frame_duration: 0.05,
        },
    );
    
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: Handle::weak_from_u128(0), // You'll need to properly handle this
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player::default(),
        animation,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player, &mut SpriteSheetAnimation)>,
    time: Res<Time>,
) {
    for (mut transform, player, mut animation) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut is_moving = false;

        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            is_moving = true;
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            is_moving = true;
        }
        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            is_moving = true;
        }
        if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            is_moving = true;
        }

        if is_moving {
            if animation.current_animation != "walk" {
                animation.play("walk", true);
            }
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            transform.translation += direction * player.speed * time.delta_secs();
        } else if animation.current_animation != "idle" {
            animation.play("idle", true);
        }
    }
}

// src/entities/enemy.rs
use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_movement, enemy_ai));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub health: i32,
    pub direction: Vec2,
    pub detection_range: f32,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    Goblin,
    Skeleton,
    Orc,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Goblin => Self {
                enemy_type,
                speed: 150.0,
                health: 30,
                direction: Vec2::new(1.0, 0.0),
                detection_range: 200.0,
            },
            EnemyType::Skeleton => Self {
                enemy_type,
                speed: 100.0,
                health: 50,
                direction: Vec2::new(0.0, 1.0),
                detection_range: 250.0,
            },
            EnemyType::Orc => Self {
                enemy_type,
                speed: 80.0,
                health: 100,
                direction: Vec2::new(-1.0, 0.0),
                detection_range: 150.0,
            },
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    enemy_type: EnemyType,
) {
    let texture_handle: Handle<Image> = match enemy_type {
        EnemyType::Goblin => asset_server.load("sprites/goblin_sheet.png"),
        EnemyType::Skeleton => asset_server.load("sprites/skeleton_sheet.png"),
        EnemyType::Orc => asset_server.load("sprites/orc_sheet.png"),
    };
    
    let mut animation = SpriteSheetAnimation::new(0.15);
    
    animation.add_animation(
        "idle".to_string(),
        AnimationClip {
            start_index: 0,
            end_index: 3,
            frame_duration: 0.3,
        },
    );
    
    animation.add_animation(
        "walk".to_string(),
        AnimationClip {
            start_index: 4,
            end_index: 7,
            frame_duration: 0.15,
        },
    );
    
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: Handle::weak_from_u128(0),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(position),
        Enemy::new(enemy_type),
        animation,
    ));
}

fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_secs();
    }
}

fn enemy_ai(
    mut enemy_query: Query<(&Transform, &mut Enemy, &mut SpriteSheetAnimation), Without<crate::entities::player::Player>>,
    player_query: Query<&Transform, With<crate::entities::player::Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut enemy, mut animation) in enemy_query.iter_mut() {
            let distance = enemy_transform.translation.distance(player_transform.translation);
            
            if distance < enemy.detection_range {
                // Chase player
                let direction = (player_transform.translation - enemy_transform.translation).truncate().normalize();
                enemy.direction = direction;
                
                if animation.current_animation != "walk" {
                    animation.play("walk", true);
                }
            } else if animation.current_animation != "idle" {
                animation.play("idle", true);
            }
        }
    }
}

// src/entities/collectible.rs
use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_collectibles);
    }
}

#[derive(Component)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub value: i32,
}

#[derive(Clone, Copy)]
pub enum CollectibleType {
    Coin,
    Gem,
    HealthPotion,
    ManaPotion,
}

pub fn spawn_collectible(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    collectible_type: CollectibleType,
) {
    let (texture_path, value) = match collectible_type {
        CollectibleType::Coin => ("sprites/coin_sheet.png", 1),
        CollectibleType::Gem => ("sprites/gem_sheet.png", 10),
        CollectibleType::HealthPotion => ("sprites/health_potion_sheet.png", 50),
        CollectibleType::ManaPotion => ("sprites/mana_potion_sheet.png", 30),
    };
    
    let texture_handle: Handle<Image> = asset_server.load(texture_path);
    
    let mut animation = SpriteSheetAnimation::new(0.1);
    animation.add_animation(
        "spin".to_string(),
        AnimationClip {
            start_index: 0,
            end_index: 7,
            frame_duration: 0.1,
        },
    );
    animation.play("spin", true);
    
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: Handle::weak_from_u128(0),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(position),
        Collectible {
            collectible_type,
            value,
        },
        animation,
    ));
}

fn animate_collectibles(
    mut query: Query<&mut Transform, With<Collectible>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        // Add a subtle floating animation
        transform.translation.y += (time.elapsed_secs() * 2.0).sin() * 0.5;
    }
}

// src/main.rs - Updated
mod animation;
mod audio;
mod tilemap;
mod entities;
mod components;
mod resources;
mod setup;
mod systems;
mod constants;

use bevy::prelude::*;
use animation::AnimationPlugin;
use audio::AudioPlugin;
use tilemap::TilemapPlugin;
use entities::EntitiesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            AnimationPlugin,
            AudioPlugin,
            TilemapPlugin,
            EntitiesPlugin,
        ))
        .init_resource::<resources::score::Score>()
        .init_resource::<resources::star_spawn_timer::StarSpawnTimer>()
        .add_systems(Startup, setup::camera::spawn_camera)
        .run();
}
