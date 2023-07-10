use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;


pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: usize = 4; 
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const NUMBER_OF_STARS: usize = 4;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

// ECS Loop
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time)
        .run();

}



// Create component and resources. 
#[derive(Component)] 
pub struct Player{}

#[derive(Component)]
pub struct Enemy{
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star{}

#[derive(Resource)]
pub struct Score{
    pub value: u32,
}

impl Default for Score{
    fn default() -> Score{
        Score { value: 0 }
    }
    
}


#[derive(Resource)]
pub struct StarSpawnTimer{
    pub timer: Timer,
}

impl Default for StarSpawnTimer{
    fn default() -> StarSpawnTimer{
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),}
    }
}




// Player spawn function
pub fn spawn_player(mut commands: Commands, 
                    window_query: Query<&Window, With<PrimaryWindow>>,
                    asset_server: Res<AssetServer>,
                    ){
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player_x.png"),
            ..default()
        },
        Player{},
    ));           
}


// Camera spawn 
pub fn spawn_camera(mut commands: Commands, 
                    window_query: Query<&Window, With<PrimaryWindow>>,
                    ){
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        });
}


pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
                    ){


    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES{
        let rand_x: f32 = random::<f32>()*window.width(); 
        let rand_y: f32 = random::<f32>()*window.height(); 
    
        commands.spawn((
                SpriteBundle{
                    transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                    texture: asset_server.load("sprites/enemy_slime.png"),
                    ..default()
                },
                Enemy{
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );
    }
}

// Initial star spaning, not routine bonus spawns.
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ){

    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES{
        let rand_x: f32 = random::<f32>()*window.width();
        let rand_y: f32 = random::<f32>()*window.height();

        commands.spawn((
                SpriteBundle{
                    transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                    texture: asset_server.load("sprites/point_star.png"),
                    ..default()
                },
                Star{},
            ));
    }
}


// Player movement function
pub fn player_movement(
    keyboard_input:Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
                      ){
    let mut direction = Vec3::ZERO;

    if let Ok(mut transform) = player_query.get_single_mut(){
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// Prevent player from leaving window 
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ){

    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();
        
        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        if translation.x <x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }


        if translation.y <y_min{
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }
        
        player_transform.translation = translation;


    }

}

// Enemy movement function.
pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
                     ){

    for (mut transform, enemy) in enemy_query.iter_mut(){
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}


// Change enemy direction when hitting a wall.
pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
                     ){

    let window = window_query.get_single().unwrap();
    
    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;


        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }


        if direction_changed{
            let sound_effect_1: Handle<AudioSource> = asset_server.load("audio/audio_001.ogg");
            let sound_effect_2: Handle<AudioSource> = asset_server.load("audio/audio_002.ogg");


           
            let sound_effect: Handle<AudioSource> = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            
            //audio.play(sound_effect);

        }
    }   
}


// Prevent enemy from leaving window.
pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ){

    let window = window_query.get_single().unwrap();

    
    let half_enemy_size: f32 = ENEMY_SIZE/ 2.0;
    let x_min: f32 = 0.0 + half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;
            
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max{
            translation.y = y_max;
        }
        
        transform.translation = translation;
    }
}
                             
// Player and enemy collision detection.
pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
   ){
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut(){
        for enemy_transform in enemy_query.iter(){
            let distance: f32 = player_transform.translation.distance(enemy_transform.translation);
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let enemy_radius: f32 = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect: Handle<AudioSource> = asset_server.load("audio/audio_001");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
            }

        }
    }
}


// Player and star collision detection.
pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
    ){
    
    if let Ok(player_transform) = player_query.get_single(){
        for(star_entity, star_transform) in star_query.iter(){
            let distance = player_transform.translation.distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0{
                println!("Player hit star!");
                score.value += 1;
                let sound_effect: Handle<AudioSource> = asset_server.load("audio/audio_001.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}


// Update score variable.
pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("score: {}", score.value.to_string());
    }
}


// Timer tick handling star spawning.
pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>){
    star_spawn_timer.timer.tick(time.delta());
    
}

// Star spawner over time. 
pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    ){
    
    if star_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();
        let random_x: f32 = random::<f32>() * window.height();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
                SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/point_star.png"),
                    ..default()
                }, 
                Star{},
                       ));
    }
}
                            

