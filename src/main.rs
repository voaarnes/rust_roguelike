
mod animation;
mod audio;
mod tilemap;
mod entities;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Modular Bevy Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            GamePlugin,
            animation::AnimationPlugin,
            audio::AudioPlugin,
            tilemap::TilemapPlugin,
            entities::EntitiesPlugin,
        ))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .init_resource::<GameResources>()
            .add_systems(OnEnter(GameState::Loading), load_game_assets)
            .add_systems(Update, check_loading_complete.run_if(in_state(GameState::Loading)))
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(Update, game_loop.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Paused), pause_game)
            .add_systems(OnExit(GameState::Paused), resume_game);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Resource, Default)]
struct GameResources {
    score: u32,
    current_level: usize,
    player_health: i32,
}

fn load_game_assets(
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    // Load all your assets here
    loading_assets.handles.push(asset_server.load("sprites/player_sheet.png"));
    loading_assets.handles.push(asset_server.load("sprites/enemy_sheet.png"));
    loading_assets.handles.push(asset_server.load("sprites/tileset.png"));
    loading_assets.handles.push(asset_server.load("audio/background_music.ogg"));
}

#[derive(Resource, Default)]
struct LoadingAssets {
    handles: Vec<Handle<Image>>,
}

fn check_loading_complete(
    asset_server: Res<AssetServer>,
    loading_assets: Res<LoadingAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let all_loaded = loading_assets.handles.iter()
        .all(|handle| asset_server.is_loaded_with_dependencies(handle));
    
    if all_loaded {
        next_state.set(GameState::Playing);
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn camera
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
    
    // Load and spawn the level
    if let Ok(level_data) = tilemap::level_loader::load_level_from_json("assets/levels/level1.json") {
        let tileset_config = tilemap::tileset_config::TilesetConfig::default();
        tilemap::tilemap_spawner::spawn_level(
            &mut commands,
            &level_data,
            &asset_server,
            &tileset_config,
            &mut texture_atlas_layouts,
        );
    }
}

fn game_loop(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn pause_game() {
    println!("Game paused");
}

fn resume_game() {
    println!("Game resumed");
}
