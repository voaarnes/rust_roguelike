use bevy::prelude::*;
use crate::game::animation::{AnimationController, AnimationClip};

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnCollectible>()
            .init_resource::<CollectibleAssets>()
            .add_systems(Startup, load_collectible_assets)
            .add_systems(Update, (handle_spawn_events, animate_collectibles));
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

#[derive(Event)]
pub struct SpawnCollectible {
    pub position: Vec3,
    pub collectible_type: CollectibleType,
}

#[derive(Resource, Default)]
pub struct CollectibleAssets {
    pub coin_texture: Handle<Image>,
    pub gem_texture: Handle<Image>,
    pub layouts: Vec<Handle<TextureAtlasLayout>>,
}

fn load_collectible_assets(
    mut assets: ResMut<CollectibleAssets>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    assets.coin_texture = asset_server.load("sprites/meyveler.png");
    assets.gem_texture = asset_server.load("sprites/meyveler.png");
    
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        3, 3,
        None, None,
    );
    assets.layouts.push(layouts.add(layout));
}

fn handle_spawn_events(
    mut commands: Commands,
    mut events: EventReader<SpawnCollectible>,
    assets: Res<CollectibleAssets>,
) {
    for event in events.read() {
        spawn_collectible(&mut commands, &assets, event.position, event.collectible_type);
    }
}

pub fn spawn_collectible(
    commands: &mut Commands,
    assets: &CollectibleAssets,
    position: Vec3,
    collectible_type: CollectibleType,
) {
    let (texture, value, start_index) = match collectible_type {
        CollectibleType::Coin => (assets.coin_texture.clone(), 1, 0),
        CollectibleType::Gem => (assets.gem_texture.clone(), 10, 3),
        CollectibleType::HealthPotion => (assets.coin_texture.clone(), 0, 6),
        CollectibleType::ManaPotion => (assets.coin_texture.clone(), 0, 7),
    };
    
    let mut anim = AnimationController::new();
    anim.add_animation("spin", AnimationClip::new(start_index, start_index + 2, 0.1, true));
    anim.play("spin");
    
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: if !assets.layouts.is_empty() {
                Some(TextureAtlas {
                    layout: assets.layouts[0].clone(),
                    index: start_index,
                })
            } else {
                None
            },
            ..default()
        },
        Transform::from_translation(position),
        Collectible { collectible_type, value },
        anim,
    ));
}

fn animate_collectibles(
    mut query: Query<&mut Transform, With<Collectible>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y += (time.elapsed_secs() * 2.0).sin() * 0.5;
    }
}
