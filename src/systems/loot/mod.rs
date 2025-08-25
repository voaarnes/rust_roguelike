use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LootTable>()
            .init_resource::<CollectedLoot>()
            .add_event::<DropLootEvent>()
            .add_event::<CollectLootEvent>()
            .add_systems(Startup, initialize_loot_tables)
            .add_systems(Update, (
                handle_loot_drops,
                handle_loot_collection,
                apply_loot_effects,
                animate_loot_drops,
            ));
    }
}

#[derive(Component)]
pub struct LootDrop {
    pub loot_type: LootType,
    pub rarity: Rarity,
    pub value: f32,
    pub pickup_range: f32,
    pub lifetime: Timer,
    pub magnetic: bool,
}

#[derive(Clone)]
pub enum LootType {
    Currency(CurrencyType, u32),
    Equipment(Equipment),
    Consumable(ConsumableItem),
    Material(MaterialType),
    Experience(u32),
    SkillGem(String),
}

#[derive(Clone, Copy)]
pub enum CurrencyType {
    Coins,
    Gems,
    SoulShards,
}

#[derive(Clone)]
pub struct Equipment {
    pub id: String,
    pub name: String,
    pub slot: EquipmentSlot,
    pub stats: HashMap<StatType, f32>,
    pub special_effects: Vec<SpecialEffect>,
    pub set_bonus: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Accessory,
    Rune,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StatType {
    Health,
    Damage,
    Speed,
    CritChance,
    CritDamage,
    Armor,
    LifeSteal,
}

#[derive(Clone)]
pub enum SpecialEffect {
    OnHit(String, f32), // effect name, chance
    OnKill(String),
    Aura(String, f32), // aura type, radius
    Passive(String),
}

#[derive(Clone)]
pub enum ConsumableItem {
    HealthPotion(i32),
    ManaPotion(i32),
    SpeedBoost(f32, f32), // multiplier, duration
    DamageBoost(f32, f32),
    Shield(i32, f32),
    Bomb(i32, f32), // damage, radius
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialType {
    IronOre,
    MagicDust,
    DragonScale,
    SoulEssence,
    ChaosOrb,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl Rarity {
    pub fn color(&self) -> Color {
        match self {
            Rarity::Common => Color::srgb(0.7, 0.7, 0.7),
            Rarity::Uncommon => Color::srgb(0.0, 0.8, 0.0),
            Rarity::Rare => Color::srgb(0.0, 0.5, 1.0),
            Rarity::Epic => Color::srgb(0.7, 0.0, 1.0),
            Rarity::Legendary => Color::srgb(1.0, 0.5, 0.0),
            Rarity::Mythic => Color::srgb(1.0, 0.0, 0.5),
        }
    }
    
    pub fn drop_chance(&self) -> f32 {
        match self {
            Rarity::Common => 0.6,
            Rarity::Uncommon => 0.25,
            Rarity::Rare => 0.10,
            Rarity::Epic => 0.04,
            Rarity::Legendary => 0.009,
            Rarity::Mythic => 0.001,
        }
    }
}

#[derive(Resource, Default)]
pub struct LootTable {
    pub enemy_drops: HashMap<String, Vec<LootEntry>>,
    pub chest_drops: HashMap<String, Vec<LootEntry>>,
    pub boss_drops: HashMap<String, Vec<LootEntry>>,
}

#[derive(Clone)]
pub struct LootEntry {
    pub loot: LootType,
    pub weight: f32,
    pub min_wave: u32,
    pub guaranteed: bool,
}

#[derive(Resource, Default)]
pub struct CollectedLoot {
    pub total_items: HashMap<Rarity, u32>,
    pub equipment: Vec<Equipment>,
    pub materials: HashMap<MaterialType, u32>,
}

#[derive(Event)]
pub struct DropLootEvent {
    pub position: Vec3,
    pub source: LootSource,
    pub luck_bonus: f32,
}

#[derive(Clone)]
pub enum LootSource {
    Enemy(String),
    Boss(String),
    Chest(String),
    Environment,
}

#[derive(Event)]
pub struct CollectLootEvent {
    pub loot_entity: Entity,
    pub collector: Entity,
}

fn initialize_loot_tables(mut loot_table: ResMut<LootTable>) {
    // Common enemy drops
    let goblin_drops = vec![
        LootEntry {
            loot: LootType::Currency(CurrencyType::Coins, 10),
            weight: 10.0,
            min_wave: 0,
            guaranteed: true,
        },
        LootEntry {
            loot: LootType::Material(MaterialType::IronOre),
            weight: 2.0,
            min_wave: 0,
            guaranteed: false,
        },
        LootEntry {
            loot: LootType::Consumable(ConsumableItem::HealthPotion(25)),
            weight: 1.0,
            min_wave: 0,
            guaranteed: false,
        },
    ];
    
    loot_table.enemy_drops.insert("goblin".to_string(), goblin_drops);
    
    // Boss drops
    let goblin_king_drops = vec![
        LootEntry {
            loot: LootType::Currency(CurrencyType::Gems, 5),
            weight: 10.0,
            min_wave: 0,
            guaranteed: true,
        },
        LootEntry {
            loot: LootType::Equipment(Equipment {
                id: "goblin_crown".to_string(),
                name: "Goblin King's Crown".to_string(),
                slot: EquipmentSlot::Armor,
                stats: HashMap::from([
                    (StatType::Health, 50.0),
                    (StatType::Damage, 10.0),
                ]),
                special_effects: vec![SpecialEffect::Aura("intimidation".to_string(), 100.0)],
                set_bonus: Some("goblin_slayer_set".to_string()),
            }),
            weight: 5.0,
            min_wave: 5,
            guaranteed: false,
        },
        LootEntry {
            loot: LootType::SkillGem("goblin_rage".to_string()),
            weight: 2.0,
            min_wave: 5,
            guaranteed: false,
        },
    ];
    
    loot_table.boss_drops.insert("goblin_king".to_string(), goblin_king_drops);
}

fn handle_loot_drops(
    mut commands: Commands,
    mut events: EventReader<DropLootEvent>,
    loot_table: Res<LootTable>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let drops = match &event.source {
            LootSource::Enemy(enemy_type) => loot_table.enemy_drops.get(enemy_type),
            LootSource::Boss(boss_type) => loot_table.boss_drops.get(boss_type),
            LootSource::Chest(chest_type) => loot_table.chest_drops.get(chest_type),
            LootSource::Environment => None,
        };
        
        if let Some(drop_list) = drops {
            let mut rng = rand::thread_rng();
            
            for entry in drop_list {
                if entry.guaranteed || rng.gen::<f32>() < calculate_drop_chance(entry.weight, event.luck_bonus) {
                    let rarity = determine_rarity(event.luck_bonus);
                    spawn_loot_drop(&mut commands, event.position, entry.loot.clone(), rarity, &asset_server);
                }
            }
        }
    }
}

fn calculate_drop_chance(base_weight: f32, luck_bonus: f32) -> f32 {
    (base_weight * (1.0 + luck_bonus)).min(1.0)
}

fn determine_rarity(luck_bonus: f32) -> Rarity {
    let mut rng = rand::thread_rng();
    let roll = rng.gen::<f32>() * (1.0 - luck_bonus * 0.1);
    
    if roll < Rarity::Mythic.drop_chance() {
        Rarity::Mythic
    } else if roll < Rarity::Legendary.drop_chance() {
        Rarity::Legendary
    } else if roll < Rarity::Epic.drop_chance() {
        Rarity::Epic
    } else if roll < Rarity::Rare.drop_chance() {
        Rarity::Rare
    } else if roll < Rarity::Uncommon.drop_chance() {
        Rarity::Uncommon
    } else {
        Rarity::Common
    }
}

fn spawn_loot_drop(
    commands: &mut Commands,
    position: Vec3,
    loot_type: LootType,
    rarity: Rarity,
    asset_server: &AssetServer,
) {
    let mut rng = rand::thread_rng();
    let offset = Vec3::new(
        rng.gen_range(-20.0..20.0),
        rng.gen_range(-20.0..20.0),
        0.0,
    );
    
    commands.spawn((
        LootDrop {
            loot_type,
            rarity,
            value: 1.0,
            pickup_range: 30.0,
            lifetime: Timer::from_seconds(30.0, TimerMode::Once),
            magnetic: false,
        },
        Sprite {
            color: rarity.color(),
            custom_size: Some(Vec2::splat(16.0)),
            ..default()
        },
        Transform::from_translation(position + offset),
    ));
}

fn handle_loot_collection(
    mut commands: Commands,
    mut collect_events: EventReader<CollectLootEvent>,
    mut loot_q: Query<&LootDrop>,
    mut collected: ResMut<CollectedLoot>,
) {
    for event in collect_events.read() {
        if let Ok(loot) = loot_q.get(event.loot_entity) {
            // Record collection
            *collected.total_items.entry(loot.rarity).or_insert(0) += 1;
            
            // Apply loot effects based on type
            match &loot.loot_type {
                LootType::Equipment(equipment) => {
                    collected.equipment.push(equipment.clone());
                }
                LootType::Material(material) => {
                    *collected.materials.entry(*material).or_insert(0) += 1;
                }
                _ => {}
            }
            
            // Remove the loot entity
            commands.entity(event.loot_entity).despawn();
        }
    }
}

fn apply_loot_effects(
    collected: Res<CollectedLoot>,
    mut player_q: Query<&mut crate::game::combat::CombatStats, With<crate::game::player::Player>>,
) {
    // Apply equipment stats and effects to player
}

fn animate_loot_drops(
    mut loot_q: Query<(&mut Transform, &mut LootDrop, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, mut loot, entity) in loot_q.iter_mut() {
        // Floating animation
        transform.translation.y += (time.elapsed_secs() * 2.0).sin() * 0.5;
        
        // Lifetime management
        loot.lifetime.tick(time.delta());
        if loot.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
