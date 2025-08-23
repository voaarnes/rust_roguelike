// src/lib.rs
pub mod game {
    pub mod player;
    pub mod enemy;
    pub mod collectible;
    pub mod combat;
    pub mod movement;
    pub mod spawning;  // Make sure only spawning.rs exists, not spawning/mod.rs
    pub mod progression;
    pub mod abilities;
    pub mod items;
    pub mod animation;
    pub mod audio;
    pub mod player_visual;
}

pub mod world {
    pub mod collision;
    pub mod tilemap;
    pub mod level_loader;
}

pub mod core;
pub mod entities;
pub mod ui;
pub mod utils;
pub mod stages;
pub mod setup;
pub mod states;
