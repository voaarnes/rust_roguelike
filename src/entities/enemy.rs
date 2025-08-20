use bevy::prelude::*;
use crate::animation::sprite_sheet::{SpriteSheetAnimation, AnimationClip};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEnemy>()
            .add_systems(Startup, (build_enemy_atlases, ApplyDeferred, seed_enemies).chain())
            .add_systems(Update, handle_spawn_enemy_events)
            .add_systems(Update, (enemy_ai, enemy_movement));
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

#[derive(Resource, Clone)]
pub struct EnemyAtlases {
    pub layout: Handle<TextureAtlasLayout>,
    pub goblin: Handle<Image>,
    pub skeleton: Handle<Image>,
    pub orc: Handle<Image>,
}

pub const ENEMY_FRAME_W: u32 = 32;
pub const ENEMY_FRAME_H: u32 = 32;
pub const ENEMY_COLUMNS: u32 = 4;
pub const ENEMY_ROWS: u32 = 2;

fn build_enemy_atlases(
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(ENEMY_FRAME_W, ENEMY_FRAME_H),
        ENEMY_COLUMNS,
        ENEMY_ROWS,
        None,
        None,
    );
    let layout_handle = layouts.add(layout);

    let goblin   = asset_server.load("sprites/tmpsprite.png");
    let skeleton = asset_server.load("sprites/tmpsprite.png");
    let orc      = asset_server.load("sprites/tmpsprite.png");

    commands.insert_resource(EnemyAtlases {
        layout: layout_handle,
        goblin,
        skeleton,
        orc,
    });
}

#[derive(Event)]
pub struct SpawnEnemy {
    pub position: Vec3,
    pub kind: EnemyType,
}

fn seed_enemies(mut writer: EventWriter<SpawnEnemy>) {
    writer.write(SpawnEnemy { position: Vec3::new(200.0, 80.0, 3.0), kind: EnemyType::Goblin });
}

fn handle_spawn_enemy_events(
    mut commands: Commands,
    atlases: Res<EnemyAtlases>,
    mut reader: EventReader<SpawnEnemy>,
) {
    for ev in reader.read() {
        spawn_enemy_entity(&mut commands, &atlases, ev.position, ev.kind);
    }
}

fn spawn_enemy_entity(
    commands: &mut Commands,
    atlases: &EnemyAtlases,
    position: Vec3,
    enemy_type: EnemyType,
) {
    let texture_handle: Handle<Image> = match enemy_type {
        EnemyType::Goblin => atlases.goblin.clone(),
        EnemyType::Skeleton => atlases.skeleton.clone(),
        EnemyType::Orc => atlases.orc.clone(),
    };

    let mut animation = SpriteSheetAnimation::new(0.15);
    animation.add_animation(
        "idle".to_string(),
        AnimationClip { start_index: 0, end_index: 3, frame_duration: 0.3 },
    );
    animation.add_animation(
        "walk".to_string(),
        AnimationClip { start_index: 4, end_index: 7, frame_duration: 0.15 },
    );
    animation.play("idle", true);

    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: atlases.layout.clone(),
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
        let dir = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        if dir.length_squared() > 0.0 {
            transform.translation += dir.normalize() * enemy.speed * time.delta_secs();
        }
    }
}

fn enemy_ai(
    mut enemy_query: Query<(&Transform, &mut Enemy, &mut SpriteSheetAnimation)>,
    player_query: Query<&Transform, With<crate::entities::player::Player>>,
) {
    if let Ok(player_tf) = player_query.single() {
        for (enemy_tf, mut enemy, mut anim) in enemy_query.iter_mut() {
            let to_player = player_tf.translation - enemy_tf.translation;
            let distance = to_player.length();
            if distance < enemy.detection_range {
                let v2 = to_player.truncate();
                if v2.length_squared() > 0.0 {
                    enemy.direction = v2.normalize();
                }
                if anim.current_animation != "walk" {
                    anim.play("walk", true);
                }
            } else if anim.current_animation != "idle" {
                anim.play("idle", true);
            }
        }
    }
}
