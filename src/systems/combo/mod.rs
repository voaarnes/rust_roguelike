use bevy::prelude::*;

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ComboTracker>()
            .add_event::<ComboEvent>()
            .add_systems(Update, (
                update_combo_timer,
                handle_combo_events,
                apply_combo_multipliers,
                trigger_combo_rewards,
            ));
    }
}

#[derive(Resource)]
pub struct ComboTracker {
    pub current_combo: u32,
    pub max_combo: u32,
    pub combo_timer: Timer,
    pub combo_multiplier: f32,
    pub total_combo_points: u64,
    pub combo_tier: ComboTier,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ComboTier {
    None,
    Bronze,   // 10+ combo
    Silver,   // 25+ combo
    Gold,     // 50+ combo
    Platinum, // 100+ combo
    Diamond,  // 200+ combo
}

impl ComboTier {
    pub fn from_combo(combo: u32) -> Self {
        match combo {
            0..=9 => ComboTier::None,
            10..=24 => ComboTier::Bronze,
            25..=49 => ComboTier::Silver,
            50..=99 => ComboTier::Gold,
            100..=199 => ComboTier::Platinum,
            _ => ComboTier::Diamond,
        }
    }
    
    pub fn multiplier(&self) -> f32 {
        match self {
            ComboTier::None => 1.0,
            ComboTier::Bronze => 1.25,
            ComboTier::Silver => 1.5,
            ComboTier::Gold => 2.0,
            ComboTier::Platinum => 3.0,
            ComboTier::Diamond => 5.0,
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            ComboTier::None => Color::WHITE,
            ComboTier::Bronze => Color::srgb(0.8, 0.5, 0.2),
            ComboTier::Silver => Color::srgb(0.8, 0.8, 0.8),
            ComboTier::Gold => Color::srgb(1.0, 0.8, 0.0),
            ComboTier::Platinum => Color::srgb(0.7, 0.7, 1.0),
            ComboTier::Diamond => Color::srgb(0.5, 1.0, 1.0),
        }
    }
}

#[derive(Event)]
pub enum ComboEvent {
    Hit,
    Kill,
    SpecialKill(String), // Special kill type (headshot, multi-kill, etc.)
    Reset,
}

impl Default for ComboTracker {
    fn default() -> Self {
        Self {
            current_combo: 0,
            max_combo: 0,
            combo_timer: Timer::from_seconds(3.0, TimerMode::Once),
            combo_multiplier: 1.0,
            total_combo_points: 0,
            combo_tier: ComboTier::None,
        }
    }
}

fn update_combo_timer(
    mut combo: ResMut<ComboTracker>,
    time: Res<Time>,
) {
    combo.combo_timer.tick(time.delta());
    
    if combo.combo_timer.finished() && combo.current_combo > 0 {
        // Combo expired
        combo.current_combo = 0;
        combo.combo_tier = ComboTier::None;
        combo.combo_multiplier = 1.0;
    }
}

fn handle_combo_events(
    mut events: EventReader<ComboEvent>,
    mut combo: ResMut<ComboTracker>,
) {
    for event in events.read() {
        match event {
            ComboEvent::Hit => {
                combo.current_combo += 1;
                combo.combo_timer.reset();
            }
            ComboEvent::Kill => {
                combo.current_combo += 2;
                combo.combo_timer.reset();
            }
            ComboEvent::SpecialKill(_) => {
                combo.current_combo += 5;
                combo.combo_timer.reset();
            }
            ComboEvent::Reset => {
                combo.current_combo = 0;
            }
        }
        
        // Update max combo
        if combo.current_combo > combo.max_combo {
            combo.max_combo = combo.current_combo;
        }
        
        // Update combo tier and multiplier
        let new_tier = ComboTier::from_combo(combo.current_combo);
        if new_tier != combo.combo_tier {
            combo.combo_tier = new_tier;
            combo.combo_multiplier = new_tier.multiplier();
            
            // Trigger visual/audio feedback for tier change
            println!("Combo Tier: {:?} ({}x multiplier)", combo.combo_tier, combo.combo_multiplier);
        }
        
        // Add to total points
        combo.total_combo_points += combo.current_combo as u64 * combo.combo_multiplier as u64;
    }
}

fn apply_combo_multipliers(
    combo: Res<ComboTracker>,
    mut player_q: Query<&mut crate::combat::CombatStats, With<crate::player::Player>>,
) {
    if let Ok(mut stats) = player_q.single_mut() {
        // Apply combo multiplier to damage
        // This would be integrated with your combat system
    }
}

fn trigger_combo_rewards(
    combo: Res<ComboTracker>,
    mut currency: ResMut<crate::systems::shop::PlayerCurrency>,
) {
    // Award bonus currency based on combo tier when combo ends
    if combo.current_combo == 0 && combo.max_combo > 0 {
        let bonus = match ComboTier::from_combo(combo.max_combo) {
            ComboTier::Bronze => 10,
            ComboTier::Silver => 25,
            ComboTier::Gold => 50,
            ComboTier::Platinum => 100,
            ComboTier::Diamond => 250,
            _ => 0,
        };
        
        if bonus > 0 {
            currency.coins += bonus;
            println!("Combo bonus: +{} coins!", bonus);
        }
    }
}
