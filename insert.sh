#!/bin/bash

# Compatible fix script for your existing Bevy 0.16 codebase
echo "Applying compatible Bevy 0.16 game fixes..."

# Fix 1: Add tile collision detection to existing movement system
cat > src/world/collision.rs << 'EOF'
use bevy::prelude::*;
use crate::world::tilemap::{Tile, TileType};
use crate::game::movement::{Velocity, Collider};
use crate::game::player::Player;

pub struct TileCollisionPlugin;

impl Plugin for TileCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_tile_collisions);
    }
}

fn check_tile_collisions(
    mut player_query: Query<(&mut Transform, &Collider, &mut Velocity), With<Player>>,
    tile_query: Query<(&Transform, &Tile), (With<Tile>, Without<Player>)>,
) {
    let Ok((mut player_transform, player_collider, mut velocity)) = player_query.single_mut() else { return };
    
    let player_pos = player_transform.translation.truncate();
    let player_half_size = player_collider.size / 2.0;
    
    for (tile_transform, tile) in tile_query.iter() {
        // Skip walkable tiles
        if tile.walkable {
            continue;
        }
        
        let tile_pos = tile_transform.translation.truncate();
        let tile_half_size = Vec2::splat(16.0); // Half of 32x32 tile
        
        // Check collision
        if (player_pos.x - player_half_size.x < tile_pos.x + tile_half_size.x) &&
           (player_pos.x + player_half_size.x > tile_pos.x - tile_half_size.x) &&
           (player_pos.y - player_half_size.y < tile_pos.y + tile_half_size.y) &&
           (player_pos.y + player_half_size.y > tile_pos.y - tile_half_size.y) {
            
            // Calculate overlap
            let overlap_x = (player_half_size.x + tile_half_size.x) - (player_pos.x - tile_pos.x).abs();
            let overlap_y = (player_half_size.y + tile_half_size.y) - (player_pos.y - tile_pos.y).abs();
            
            // Resolve collision by moving player away from tile
            if overlap_x < overlap_y {
                // Horizontal collision
                if player_pos.x < tile_pos.x {
                    player_transform.translation.x = tile_pos.x - tile_half_size.x - player_half_size.x;
                } else {
                    player_transform.translation.x = tile_pos.x + tile_half_size.x + player_half_size.x;
                }
                velocity.0.x = 0.0;
            } else {
                // Vertical collision
                if player_pos.y < tile_pos.y {
                    player_transform.translation.y = tile_pos.y - tile_half_size.y - player_half_size.y;
                } else {
                    player_transform.translation.y = tile_pos.y + tile_half_size.y + player_half_size.y;
                }
                velocity.0.y = 0.0;
            }
        }
    }
}

pub fn is_position_walkable(
    position: Vec2,
    tile_query: &Query<(&Transform, &Tile), With<Tile>>,
) -> bool {
    for (tile_transform, tile) in tile_query.iter() {
        let tile_pos = tile_transform.translation.truncate();
        let tile_half_size = Vec2::splat(16.0);
        
        // Check if position is within this tile
        if (position.x >= tile_pos.x - tile_half_size.x) &&
           (position.x <= tile_pos.x + tile_half_size.x) &&
           (position.y >= tile_pos.y - tile_half_size.y) &&
           (position.y <= tile_pos.y + tile_half_size.y) {
            return tile.walkable;
        }
    }
    true // Default to walkable if no tile found
}
EOF

# Fix 2: Update spawning system to only spawn in walkable areas
cat >> src/game/spawning.rs << 'EOF'

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
EOF

# Fix 3: Update PowerUpSlots to use FIFO queue
cat > src/entities/powerup.rs << 'EOF'
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone)]
pub struct PowerUpSlots {
    pub slots: VecDeque<PowerUpType>,
    pub max_slots: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerUpType {
    SpeedBoost,
    DamageBoost,
    HealthBoost,
    ShieldBoost,
}

impl PowerUpSlots {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: VecDeque::with_capacity(max_slots),
            max_slots,
        }
    }
    
    pub fn add_powerup(&mut self, powerup: PowerUpType) -> Option<PowerUpType> {
        let dropped = if self.slots.len() >= self.max_slots {
            self.slots.pop_back() // Remove oldest (FIFO)
        } else {
            None
        };
        
        self.slots.push_front(powerup); // Add newest to front
        dropped
    }
    
    pub fn get_slots_as_vec(&self) -> Vec<Option<PowerUpType>> {
        let mut vec = Vec::with_capacity(self.max_slots);
        for i in 0..self.max_slots {
            if i < self.slots.len() {
                vec.push(Some(self.slots[i]));
            } else {
                vec.push(None);
            }
        }
        vec
    }
    
    pub fn get_head_fruit(&self) -> Option<PowerUpType> {
        self.slots.front().copied()
    }
    
    pub fn get_legs_fruit(&self) -> Option<PowerUpType> {
        if self.slots.len() >= 2 {
            self.slots.get(self.slots.len() - 1).copied()
        } else {
            None
        }
    }
}
EOF

# Fix 4: Update collectible system to use FIFO queue  
cat >> src/game/collectible.rs << 'EOF'

// Update fruit pickup to use FIFO queue
fn handle_fruit_pickup(
    mut commands: Commands,
    mut player_q: Query<(&Transform, &mut crate::game::player::PlayerStats), With<crate::game::player::Player>>,
    collectible_q: Query<(Entity, &Transform, &Collectible, &crate::game::movement::Collider)>,
    mut powerup_q: Query<&mut crate::entities::powerup::PowerUpSlots, With<crate::game::player::Player>>,
) {
    let Ok((player_tf, mut player_stats)) = player_q.single_mut() else { return };
    
    for (collectible_entity, collectible_tf, collectible, _collider) in collectible_q.iter() {
        let distance = player_tf.translation.distance(collectible_tf.translation);
        
        // Check if close enough to pick up (within player + collectible radius)
        if distance < 40.0 {
            match collectible.collectible_type {
                CollectibleType::Coin => {
                    player_stats.coins_collected += collectible.value as u32;
                    println!("Picked up {} coins! Total: {}", collectible.value, player_stats.coins_collected);
                }
                CollectibleType::Fruit(fruit_type) => {
                    if let Ok(mut powerup_slots) = powerup_q.single_mut() {
                        let powerup = match fruit_type {
                            0 | 1 => crate::entities::powerup::PowerUpType::SpeedBoost,      // Strawberry, Pear
                            2 | 3 => crate::entities::powerup::PowerUpType::DamageBoost,     // Mango, Apple
                            4 | 5 => crate::entities::powerup::PowerUpType::HealthBoost,     // Orange, Grape
                            6 | 7 => crate::entities::powerup::PowerUpType::ShieldBoost,     // Banana, Cherry
                            _ => crate::entities::powerup::PowerUpType::SpeedBoost,
                        };
                        
                        // Use FIFO queue to add powerup
                        if let Some(dropped) = powerup_slots.add_powerup(powerup) {
                            println!("Gained power-up: {:?}, dropped: {:?}", powerup, dropped);
                        } else {
                            println!("Gained power-up: {:?}", powerup);
                        }
                    }
                }
                _ => {}
            }
            
            // Remove the collectible
            commands.entity(collectible_entity).despawn();
        }
    }
}
EOF

# Fix 5: Create player visual customization system
cat > src/game/player_visual.rs << 'EOF'
use bevy::prelude::*;
use crate::game::player::Player;
use crate::entities::powerup::{PowerUpSlots, PowerUpType};

pub struct PlayerVisualPlugin;

impl Plugin for PlayerVisualPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player_parts)
            .add_systems(Update, update_player_appearance);
    }
}

#[derive(Component)]
pub struct PlayerParts {
    pub head_entity: Option<Entity>,
    pub chest_entity: Option<Entity>,
    pub legs_entity: Option<Entity>,
}

#[derive(Component)]
pub struct PlayerPartType {
    pub part_type: PartType,
}

#[derive(Clone, Copy)]
pub enum PartType {
    Head,
    Chest,
    Legs,
}

fn setup_player_parts(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok(player_entity) = player_query.single() else { return };
    
    let texture = asset_server.load("sprites/player_parts.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        8, 6,
        None, None,
    );
    let layout_handle = layouts.add(layout);
    
    // Spawn head (default grape design at index 6)
    let head_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 6, // Grape head as default
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 1.0), // Slightly above center
        PlayerPartType { part_type: PartType::Head },
    )).id();
    
    // Spawn chest (default grape design at index 22)
    let chest_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 22, // Grape chest as default
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0), // Center
        PlayerPartType { part_type: PartType::Chest },
    )).id();
    
    // Spawn legs (default grape design at index 38)
    let legs_entity = commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: 38, // Grape legs as default
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -8.0, 1.0), // Slightly below center
        PlayerPartType { part_type: PartType::Legs },
    )).id();
    
    // Add PlayerParts component to player
    commands.entity(player_entity).insert(PlayerParts {
        head_entity: Some(head_entity),
        chest_entity: Some(chest_entity),
        legs_entity: Some(legs_entity),
    });
    
    // Make parts children of player
    commands.entity(player_entity).add_children(&[head_entity, chest_entity, legs_entity]);
}

fn update_player_appearance(
    player_query: Query<(&PlayerParts, &PowerUpSlots, &Transform), With<Player>>,
    mut part_query: Query<&mut Sprite>,
) {
    let Ok((player_parts, powerup_slots, _player_transform)) = player_query.single() else { return };
    
    // Get head fruit (newest/first in queue)
    let head_fruit = powerup_slots.get_head_fruit();
    
    // Get legs fruit (oldest/last in queue)
    let legs_fruit = powerup_slots.get_legs_fruit();
    
    // Update head appearance
    if let Some(head_entity) = player_parts.head_entity {
        if let Ok(mut sprite) = part_query.get_mut(head_entity) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = get_head_sprite_index(head_fruit);
            }
        }
    }
    
    // Update chest appearance (use head fruit for now)
    if let Some(chest_entity) = player_parts.chest_entity {
        if let Ok(mut sprite) = part_query.get_mut(chest_entity) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = get_chest_sprite_index(head_fruit);
            }
        }
    }
    
    // Update legs appearance
    if let Some(legs_entity) = player_parts.legs_entity {
        if let Ok(mut sprite) = part_query.get_mut(legs_entity) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = get_legs_sprite_index(legs_fruit);
            }
        }
    }
}

fn get_head_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 1,      // Strawberry head
        Some(PowerUpType::DamageBoost) => 4,     // Apple head
        Some(PowerUpType::HealthBoost) => 5,     // Orange head
        Some(PowerUpType::ShieldBoost) => 7,     // Banana head
        None => 6,                                // Grape head (default)
    }
}

fn get_chest_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 17,     // Strawberry chest
        Some(PowerUpType::DamageBoost) => 20,    // Apple chest
        Some(PowerUpType::HealthBoost) => 21,    // Orange chest
        Some(PowerUpType::ShieldBoost) => 23,    // Banana chest
        None => 22,                               // Grape chest (default)
    }
}

fn get_legs_sprite_index(powerup: Option<PowerUpType>) -> usize {
    match powerup {
        Some(PowerUpType::SpeedBoost) => 33,     // Strawberry legs
        Some(PowerUpType::DamageBoost) => 36,    // Apple legs
        Some(PowerUpType::HealthBoost) => 37,    // Orange legs
        Some(PowerUpType::ShieldBoost) => 39,    // Banana legs
        None => 38,                               // Grape legs (default)
    }
}
EOF

# Fix 6: Update tilemap to have proper walkability
cat >> src/world/tilemap.rs << 'EOF'

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
    Portal,
    Lava,
    Grass,
    Stone,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        let walkable = match tile_type {
            TileType::Floor | TileType::Door | TileType::Grass | TileType::Stone => true,
            TileType::Wall | TileType::Chest | TileType::Water | TileType::Lava => false,
            TileType::Spike | TileType::Portal => true, // Walkable but may have effects
        };
        
        let tile_index = match tile_type {
            TileType::Floor => 1,
            TileType::Grass => 2,
            TileType::Stone => 3,
            TileType::Wall => 17,
            TileType::Door => 33,
            TileType::Chest => 37,
            TileType::Spike => 41,
            TileType::Water => 45,
            TileType::Lava => 49,
            TileType::Portal => 53,
        };
        
        Self {
            tile_type,
            walkable,
            tile_index,
        }
    }
}
EOF

# Fix 7: Update UI to work with FIFO queue
cat >> src/ui/powerup_display.rs << 'EOF'

fn update_powerup_display_with_fifo(
    player_query: Query<&crate::entities::powerup::PowerUpSlots, With<crate::game::player::Player>>,
    mut slot_query: Query<(&PowerUpSlotUI, &mut BackgroundColor)>,
) {
    if let Ok(powerup_slots) = player_query.single() {
        let slots_vec = powerup_slots.get_slots_as_vec();
        
        for (slot_ui, mut bg_color) in slot_query.iter_mut() {
            if slot_ui.slot_index < slots_vec.len() {
                *bg_color = match slots_vec[slot_ui.slot_index] {
                    Some(crate::entities::powerup::PowerUpType::SpeedBoost) => BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.0)),
                    Some(crate::entities::powerup::PowerUpType::DamageBoost) => BackgroundColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                    Some(crate::entities::powerup::PowerUpType::HealthBoost) => BackgroundColor(Color::linear_rgb(0.0, 0.0, 1.0)),
                    Some(crate::entities::powerup::PowerUpType::ShieldBoost) => BackgroundColor(Color::linear_rgb(1.0, 1.0, 0.0)),
                    None => BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.8)),
                };
            }
        }
    }
}
EOF

# Fix 8: Add the new modules to your existing game mod.rs
cat >> src/game/mod.rs << 'EOF'

// Add player visual module
pub mod player_visual;
EOF

# Fix 9: Add world modules to lib.rs if not present
if ! grep -q "pub mod world" src/lib.rs; then
    cat >> src/lib.rs << 'EOF'

pub mod world {
    pub mod collision;
    pub mod tilemap;
    pub mod level_loader;
}
EOF
fi

# Fix 10: Update main.rs to include collision plugin
cat >> src/main.rs << 'EOF'

// Add this to your main.rs plugin list
// .add_plugins(crate::world::collision::TileCollisionPlugin)
// .add_plugins(crate::game::player_visual::PlayerVisualPlugin)
EOF

# Fix 11: Create sprite guides for assets
cat > assets/sprites/player_parts_guide.txt << 'EOF'
PLAYER PARTS SPRITE SHEET GUIDE (player_parts.png)
====================================================
Create a 256x192 pixel sprite sheet (8 columns x 6 rows of 32x32 sprites)

Layout (48 total sprites):
Row 0 (indices 0-7): Head variations for fruits
- 0: Default head
- 1: Strawberry head (red with seeds pattern)
- 2: Pear head (green/yellow gradient)
- 3: Mango head (orange/red gradient)
- 4: Apple head (shiny red/green)
- 5: Orange head (bright orange with texture)
- 6: Grape head (purple cluster pattern) - DEFAULT
- 7: Banana head (yellow with slight curve)

Row 1 (indices 8-15): More head variations
- 8: Cherry head (dark red with stem detail)
- 9-15: Future expansions or animation frames

Row 2 (indices 16-23): Chest/torso variations
- 16: Default chest
- 17: Strawberry chest (red with seeds pattern)
- 18: Pear chest (green/yellow gradient)
- 19: Mango chest (orange/red gradient)
- 20: Apple chest (shiny red/green)
- 21: Orange chest (bright orange with texture)
- 22: Grape chest (purple cluster pattern) - DEFAULT
- 23: Banana chest (yellow with slight curve)

Row 3 (indices 24-31): More chest variations
- 24: Cherry chest (dark red with stem detail)
- 25-31: Future expansions or animation frames

Row 4 (indices 32-39): Legs variations
- 32: Default legs
- 33: Strawberry legs (red with seeds pattern)
- 34: Pear legs (green/yellow gradient)
- 35: Mango legs (orange/red gradient)
- 36: Apple legs (shiny red/green)
- 37: Orange legs (bright orange with texture)
- 38: Grape legs (purple cluster pattern) - DEFAULT
- 39: Banana legs (yellow with slight curve)

Row 5 (indices 40-47): More legs variations
- 40: Cherry legs (dark red with stem detail)
- 41-47: Future expansions or animation frames

Each fruit should have a distinct visual theme that carries through head, chest, and legs.
The grape design (indices 6, 22, 38) should be used as the default when no fruits are equipped.
EOF

cat > assets/sprites/fruits_guide.txt << 'EOF'
FRUITS SPRITE SHEET GUIDE (fruits.png)
=======================================
Create a 256x32 pixel sprite sheet (8 columns x 1 row of 32x32 sprites)

Fruit order (left to right):
0. Strawberry - Red with green top and seeds
1. Pear - Green/yellow gradient with characteristic shape
2. Mango - Orange/red gradient with oval shape
3. Apple - Red or green with shine highlight
4. Orange - Bright orange with textured surface
5. Grape - Purple cluster of small round grapes
6. Banana - Yellow curved fruit
7. Cherry - Red pair with green stems

Each fruit should be centered in its 32x32 cell with a slight glow or outline effect for visibility.
These correspond to the PowerUpType enum:
- 0,1 (Strawberry, Pear) = SpeedBoost
- 2,3 (Mango, Apple) = DamageBoost  
- 4,5 (Orange, Grape) = HealthBoost
- 6,7 (Banana, Cherry) = ShieldBoost
EOF

# Fix 12: Add rand dependency if missing
if ! grep -q "rand" Cargo.toml; then
    echo '' >> Cargo.toml
    echo 'rand = "0.8"' >> Cargo.toml
fi

echo ""
echo "🎮 Compatible fixes applied successfully! 🎮"
echo ""
echo "Fixed issues:"
echo "✅ Tile collision detection - players can't walk through walls"
echo "✅ Enemy spawning only in walkable areas"
echo "✅ FIFO powerup queue system - new fruits push out old ones"
echo "✅ Player character visual customization based on fruits"
echo "✅ Grape design as default (no fruit equipped)"
echo "✅ Head shows newest fruit, legs show oldest fruit"
echo ""
echo "Manual steps needed:"
echo "1. Add these plugins to your main.rs plugin list:"
echo "   .add_plugins(crate::world::collision::TileCollisionPlugin)"
echo "   .add_plugins(crate::game::player_visual::PlayerVisualPlugin)"
echo ""
echo "2. Update your collectible system to call handle_fruit_pickup"
echo ""
echo "3. Update your spawning system to use find_walkable_spawn_position"
echo ""
echo "4. Create the sprite sheets according to the guides in assets/sprites/"
echo ""
echo "5. Update UI system to use update_powerup_display_with_fifo"
echo ""
echo "To test the game:"
echo "1. cargo build"
echo "2. cargo run"
echo "3. Pick up fruits and watch your character design change!"
echo "4. Notice how new fruits replace old ones (FIFO queue)"
echo "5. Try walking into walls - you should be blocked by collision"
