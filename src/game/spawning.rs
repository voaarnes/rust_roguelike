
// Add walkable area checking to spawning
use crate::world::collision::is_position_walkable;
use crate::world::tilemap::Tile;

fn find_walkable_spawn_position(
    tile_query: &Query<(&Transform, &Tile), With<Tile>>,
) -> Vec3 {
    let mut rng = rand::thread_rng();
    
    // Try up to 20 times to find a walkable position
    for _ in 0..20 {
        let x = rng.gen_range(-500.0..500.0);
        let y = rng.gen_range(-500.0..500.0);
        let pos = Vec2::new(x, y);
        
        if is_position_walkable(pos, tile_query) {
            return Vec3::new(x, y, 1.0);
        }
    }
    
    // Fallback to origin if no walkable position found
    Vec3::new(0.0, 0.0, 1.0)
}
