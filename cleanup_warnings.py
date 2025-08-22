#!/usr/bin/env python3
import os
import re

fixes = {
    'src/entities/mod.rs': [
        ('pub use crate::game::player;', '// pub use crate::game::player;'),
        ('pub use crate::game::enemy;', '// pub use crate::game::enemy;'),
        ('pub use crate::game::collectible::*;', '// pub use crate::game::collectible::*;'),
    ],
    'src/stages/stage_transition.rs': [
        ('use bevy::prelude::*;', '// use bevy::prelude::*;'),
    ],
    'src/game/combat/projectiles.rs': [
        ('use crate::core::events::{CombatEvent, DamageType};', 'use crate::core::events::DamageType;'),
    ],
    'src/game/combat/mod.rs': [
        ('use crate::core::events::{CombatEvent, DamageType};', 'use crate::core::events::CombatEvent;'),
    ],
}

for filepath, replacements in fixes.items():
    if os.path.exists(filepath):
        with open(filepath, 'r') as f:
            content = f.read()
        
        for old, new in replacements:
            content = content.replace(old, new)
        
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Fixed warnings in: {filepath}")

print("\nWarnings cleaned up!")
