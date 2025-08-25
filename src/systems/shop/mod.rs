use bevy::prelude::*;
use std::collections::HashMap;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShopInventory>()
            .init_resource::<PlayerCurrency>()
            .init_resource::<PurchasedUpgrades>()
            .add_event::<PurchaseEvent>()
            .add_systems(Startup, initialize_shop)
            .add_systems(Update, (
                handle_purchases,
                apply_upgrade_effects,
                refresh_shop_on_wave_clear,
            ));
    }
}

#[derive(Resource, Component)]
pub struct PlayerCurrency {
    pub coins: u32,
    pub gems: u32,
    pub soul_shards: u32, // Premium currency for meta-progression
}

#[derive(Resource, Default)]
pub struct ShopInventory {
    pub items: Vec<ShopItem>,
    pub refresh_cost: u32,
    pub last_refresh_wave: u32,
}

#[derive(Clone)]
pub struct ShopItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub cost: u32,
    pub currency_type: CurrencyType,
    pub upgrade_type: UpgradeType,
    pub tier: ItemTier,
    pub stock: i32, // -1 for unlimited
    pub requirements: Vec<PurchaseRequirement>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CurrencyType {
    Coins,
    Gems,
    SoulShards,
}

#[derive(Clone)]
pub enum UpgradeType {
    StatBoost(StatType, f32),
    PercentBoost(StatType, f32),
    UnlockAbility(String),
    PassiveEffect(PassiveType),
    ConsumableItem(ConsumableType),
}

#[derive(Clone, Copy)]
pub enum StatType {
    Health,
    Damage,
    Speed,
    AttackSpeed,
    CritChance,
    CritDamage,
    Armor,
    LifeSteal,
    ExperienceGain,
    CoinGain,
}

#[derive(Clone)]
pub enum PassiveType {
    Thorns(f32),           // Reflect damage
    Regeneration(f32),     // Health per second
    MagnetRange(f32),      // Item pickup range
    DodgeChance(f32),      // Chance to avoid damage
    ExecuteThreshold(f32), // Instant kill below % hp
}

#[derive(Clone)]
pub enum ConsumableType {
    HealthPotion(i32),
    ReviveToken,
    RerollShop,
    SkipWave,
}

#[derive(Clone, Copy)]
pub enum ItemTier {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Clone)]
pub enum PurchaseRequirement {
    WaveReached(u32),
    ItemOwned(String),
    AchievementUnlocked(String),
}

#[derive(Resource, Default)]
pub struct PurchasedUpgrades {
    pub upgrades: HashMap<String, u32>, // id -> stack count
}

#[derive(Event)]
pub struct PurchaseEvent {
    pub item_id: String,
    pub player: Entity,
}

impl Default for PlayerCurrency {
    fn default() -> Self {
        Self {
            coins: 100,
            gems: 0,
            soul_shards: 0,
        }
    }
}

fn initialize_shop(mut shop: ResMut<ShopInventory>) {
    shop.items = generate_shop_items();
    shop.refresh_cost = 50;
}

fn generate_shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem {
            id: "health_boost_1".to_string(),
            name: "Vitality Crystal".to_string(),
            description: "+25 Max Health".to_string(),
            icon: "icons/health.png".to_string(),
            cost: 100,
            currency_type: CurrencyType::Coins,
            upgrade_type: UpgradeType::StatBoost(StatType::Health, 25.0),
            tier: ItemTier::Common,
            stock: -1,
            requirements: vec![],
        },
        ShopItem {
            id: "damage_boost_1".to_string(),
            name: "Sharpening Stone".to_string(),
            description: "+5 Damage".to_string(),
            icon: "icons/damage.png".to_string(),
            cost: 150,
            currency_type: CurrencyType::Coins,
            upgrade_type: UpgradeType::StatBoost(StatType::Damage, 5.0),
            tier: ItemTier::Common,
            stock: -1,
            requirements: vec![],
        },
        ShopItem {
            id: "crit_boost_1".to_string(),
            name: "Lucky Charm".to_string(),
            description: "+10% Critical Chance".to_string(),
            icon: "icons/crit.png".to_string(),
            cost: 200,
            currency_type: CurrencyType::Coins,
            upgrade_type: UpgradeType::PercentBoost(StatType::CritChance, 0.1),
            tier: ItemTier::Uncommon,
            stock: -1,
            requirements: vec![],
        },
        ShopItem {
            id: "lifesteal_1".to_string(),
            name: "Vampiric Fang".to_string(),
            description: "+5% Life Steal".to_string(),
            icon: "icons/lifesteal.png".to_string(),
            cost: 300,
            currency_type: CurrencyType::Coins,
            upgrade_type: UpgradeType::PercentBoost(StatType::LifeSteal, 0.05),
            tier: ItemTier::Rare,
            stock: 1,
            requirements: vec![PurchaseRequirement::WaveReached(5)],
        },
        ShopItem {
            id: "thorns_1".to_string(),
            name: "Thorned Armor".to_string(),
            description: "Reflect 20% damage to attackers".to_string(),
            icon: "icons/thorns.png".to_string(),
            cost: 400,
            currency_type: CurrencyType::Coins,
            upgrade_type: UpgradeType::PassiveEffect(PassiveType::Thorns(0.2)),
            tier: ItemTier::Rare,
            stock: 1,
            requirements: vec![PurchaseRequirement::WaveReached(10)],
        },
    ]
}

fn handle_purchases(
    mut events: EventReader<PurchaseEvent>,
    mut currency: ResMut<PlayerCurrency>,
    mut purchased: ResMut<PurchasedUpgrades>,
    mut shop: ResMut<ShopInventory>,
) {
    for event in events.read() {
        if let Some(item) = shop.items.iter_mut().find(|i| i.id == event.item_id) {
            // Check stock
            if item.stock == 0 {
                continue;
            }
            
            // Check currency
            let can_afford = match item.currency_type {
                CurrencyType::Coins => currency.coins >= item.cost,
                CurrencyType::Gems => currency.gems >= item.cost,
                CurrencyType::SoulShards => currency.soul_shards >= item.cost,
            };
            
            if can_afford {
                // Deduct currency
                match item.currency_type {
                    CurrencyType::Coins => currency.coins -= item.cost,
                    CurrencyType::Gems => currency.gems -= item.cost,
                    CurrencyType::SoulShards => currency.soul_shards -= item.cost,
                }
                
                // Reduce stock
                if item.stock > 0 {
                    item.stock -= 1;
                }
                
                // Record purchase
                *purchased.upgrades.entry(event.item_id.clone()).or_insert(0) += 1;
                
                // Increase cost for stackable items
                item.cost = (item.cost as f32 * 1.15) as u32;
            }
        }
    }
}

fn apply_upgrade_effects(
    purchased: Res<PurchasedUpgrades>,
    shop: Res<ShopInventory>,
    mut player_q: Query<&mut crate::game::combat::CombatStats, With<crate::game::player::Player>>,
) {
    // Apply all purchased upgrades to player stats
}

fn refresh_shop_on_wave_clear(
    wave_manager: Res<crate::game::spawning::WaveManager>,
    mut shop: ResMut<ShopInventory>,
) {
    if wave_manager.wave_complete && wave_manager.current_wave != shop.last_refresh_wave {
        shop.last_refresh_wave = wave_manager.current_wave;
        // Add new items based on wave
        if wave_manager.current_wave % 5 == 0 {
            // Add special items after boss waves
        }
    }
}
