use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnCollectible>()
            .add_systems(Startup, (build_collectible_atlas, ApplyDeferred, seed_collectibles).chain())
            .add_systems(Update, handle_spawn_collectible_events)
            .add_systems(Update, animate_collectibles);
    }
}

#[derive(Component)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub value: i32,
}

#[derive(Clone, Copy)]
pub enum CollectibleType {
    Strawberry,
    Pear,
    Mango,
}

#[derive(Resource, Clone)]
pub struct FruitAtlases {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

pub const FRUIT_FRAME_W: u32 = 32;
pub const FRUIT_FRAME_H: u32 = 32;
pub const FRUIT_COLUMNS: u32 = 3;
pub const FRUIT_ROWS: u32 = 3;

pub fn build_collectible_atlas(
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(FRUIT_FRAME_W, FRUIT_FRAME_H),
        FRUIT_COLUMNS,
        FRUIT_ROWS,
        None,
        None,
    );
    let layout_handle = layouts.add(layout);
    let texture_handle: Handle<Image> = asset_server.load("sprites/meyveler.png");

    commands.insert_resource(FruitAtlases {
        layout: layout_handle,
        texture: texture_handle,
    });
}

#[derive(Event)]
pub struct SpawnCollectible {
    pub position: Vec3,
    pub kind: CollectibleType,
}

fn seed_collectibles(mut writer: EventWriter<SpawnCollectible>) {
    writer.write(SpawnCollectible { position: Vec3::new(-64.0, 0.0, 2.0), kind: CollectibleType::Strawberry });
    writer.write(SpawnCollectible { position: Vec3::new(  0.0, 0.0, 2.0), kind: CollectibleType::Pear });
    writer.write(SpawnCollectible { position: Vec3::new( 64.0, 0.0, 2.0), kind: CollectibleType::Mango });
}

fn handle_spawn_collectible_events(
    mut commands: Commands,
    atlases: Res<FruitAtlases>,
    mut reader: EventReader<SpawnCollectible>,
) {
    for ev in reader.read() {
        spawn_collectible(&mut commands, &atlases, ev.position, ev.kind);
    }
}

pub fn spawn_collectible(
    commands: &mut Commands,
    atlases: &Res<FruitAtlases>,
    position: Vec3,
    collectible_type: CollectibleType,
) {
    let (start_index, end_index, value) = match collectible_type {
        CollectibleType::Strawberry => (0, 2, 5),
        CollectibleType::Pear       => (3, 5, 10),
        CollectibleType::Mango      => (6, 8, 15),
    };

    let mut animation = SpriteSheetAnimation::new(0.1);
    animation.add_animation(
        "spin".to_string(),
        AnimationClip {
            start_index,
            end_index,
            frame_duration: 0.12,
        },
    );
    animation.play("spin", true);

    commands.spawn((
        Sprite {
            image: atlases.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlases.layout.clone(),
                index: start_index,
            }),
            ..default()
        },
        Transform::from_translation(position),
        Collectible { collectible_type, value },
        animation,
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
