pub mod shop;
pub mod talents;
pub mod achievements;
pub mod loot;
pub mod combo;
pub mod quests;
pub mod prestige;

use bevy::prelude::*;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                shop::ShopPlugin,
                talents::TalentTreePlugin,
                achievements::AchievementPlugin,
                loot::LootPlugin,
                combo::ComboPlugin,
                quests::QuestPlugin,
                prestige::PrestigePlugin,
            ));
    }
}

#[cfg(target_os = "android")]
pub mod mobile;
