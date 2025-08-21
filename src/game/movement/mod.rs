use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_velocity,
            handle_collisions,
            update_collision_grid,
        ).chain());
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Static;

#[derive(Resource, Default)]
pub struct CollisionGrid {
    pub cells: Vec<Vec<Vec<Entity>>>,
    pub cell_size: f32,
    pub width: usize,
    pub height: usize,
}

impl CollisionGrid {
    pub fn new(world_width: f32, world_height: f32, cell_size: f32) -> Self {
        let width = (world_width / cell_size).ceil() as usize;
        let height = (world_height / cell_size).ceil() as usize;
        Self {
            cells: vec![vec![Vec::new(); width]; height],
            cell_size,
            width,
            height,
        }
    }
    
    pub fn get_nearby_entities(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        let mut entities = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_x = (position.x / self.cell_size) as i32;
        let center_y = (position.y / self.cell_size) as i32;
        
        for y in (center_y - cell_radius)..=(center_y + cell_radius) {
            for x in (center_x - cell_radius)..=(center_x + cell_radius) {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    entities.extend(&self.cells[y as usize][x as usize]);
                }
            }
        }
        
        entities
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), Without<Static>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn handle_collisions(
    mut movable_q: Query<(Entity, &mut Transform, &Collider, &Velocity), Without<Static>>,
    static_q: Query<(&Transform, &Collider), With<Static>>,
    collision_grid: Res<CollisionGrid>,
) {
    for (entity, mut transform, collider, velocity) in movable_q.iter_mut() {
        let nearby = collision_grid.get_nearby_entities(transform.translation.truncate(), 100.0);
        
        for (static_tf, static_collider) in static_q.iter() {
            if check_collision(
                transform.translation.truncate(),
                collider.size,
                static_tf.translation.truncate(),
                static_collider.size,
            ) {
                // Simple push-back collision
                let diff = transform.translation - static_tf.translation;
                let overlap = (collider.size + static_collider.size) / 2.0 - diff.truncate().abs();
                
                if overlap.x > 0.0 && overlap.y > 0.0 {
                    if overlap.x < overlap.y {
                        transform.translation.x += overlap.x * diff.x.signum();
                    } else {
                        transform.translation.y += overlap.y * diff.y.signum();
                    }
                }
            }
        }
    }
}

fn update_collision_grid(
    mut grid: ResMut<CollisionGrid>,
    query: Query<(Entity, &Transform, &Collider)>,
) {
    // Clear grid
    for row in grid.cells.iter_mut() {
        for cell in row.iter_mut() {
            cell.clear();
        }
    }
    
    // Populate grid
    for (entity, transform, _) in query.iter() {
        let x = ((transform.translation.x + 1000.0) / grid.cell_size) as usize;
        let y = ((transform.translation.y + 1000.0) / grid.cell_size) as usize;
        
        if x < grid.width && y < grid.height {
            grid.cells[y][x].push(entity);
        }
    }
}

fn check_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let half1 = size1 / 2.0;
    let half2 = size2 / 2.0;
    
    (pos1.x - half1.x < pos2.x + half2.x) &&
    (pos1.x + half1.x > pos2.x - half2.x) &&
    (pos1.y - half1.y < pos2.y + half2.y) &&
    (pos1.y + half1.y > pos2.y - half2.y)
}
