use bevy::prelude::*;
use crate::systems::talents::{TalentTree, PlayerTalents, TalentTreeType, UnlockTalentEvent};

#[derive(Component)]
pub struct TalentNode {
    pub talent_id: String,
    pub tree_type: TalentTreeType,
}

pub fn render_talent_tab(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    talent_tree: &TalentTree,
    player_talents: &PlayerTalents,
) {
    // Header
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(60.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).with_children(|header| {
        header.spawn((
            Text::new("⭐ Talent Trees"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(0.5, 1.0, 0.5)),
        ));
        
        header.spawn((
            Text::new(format!("Available Points: {}", player_talents.available_points)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 0.843, 0.0)),
        ));
    });
    
    // Talent trees container
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(20.0),
        ..default()
    }).with_children(|trees| {
        for (tree_type, tree_data) in &talent_tree.trees {
            spawn_talent_tree(trees, tree_type, tree_data, player_talents);
        }
    });
}

fn spawn_talent_tree(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    tree_type: &TalentTreeType,
    tree_data: &crate::systems::talents::TreeData,
    player_talents: &PlayerTalents,
) {
    parent.spawn(Node {
        flex_grow: 1.0,
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(15.0)),
        ..default()
    }).with_children(|tree| {
        // Tree name
        tree.spawn((
            Text::new(&tree_data.name),
            TextFont { font_size: 24.0, ..default() },
            TextColor(match tree_type {
                TalentTreeType::Offense => Color::srgb(1.0, 0.2, 0.2),
                TalentTreeType::Defense => Color::srgb(0.2, 0.5, 1.0),
                TalentTreeType::Utility => Color::srgb(0.8, 0.8, 0.2),
            }),
        ));
        
        // Points spent in tree
        let points_spent = player_talents.spent_points.get(tree_type).copied().unwrap_or(0);
        tree.spawn((
            Text::new(format!("Points: {}", points_spent)),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
        
        // Talent nodes
        tree.spawn(Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        }).with_children(|nodes| {
            for (talent_id, talent) in &tree_data.talents {
                spawn_talent_node(nodes, talent, tree_type, player_talents);
            }
        });
    });
}

fn spawn_talent_node(
    parent: &mut impl bevy::hierarchy::BuildChildren,
    talent: &crate::systems::talents::Talent,
    tree_type: &TalentTreeType,
    player_talents: &PlayerTalents,
) {
    let current_rank = player_talents.unlocked_talents
        .get(&talent.id)
        .copied()
        .unwrap_or(0);
    
    let is_available = player_talents.available_points >= talent.cost_per_rank 
        && current_rank < talent.max_ranks;
    
    parent.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(if current_rank > 0 {
            Color::srgb(0.1, 0.3, 0.1)
        } else if is_available {
            Color::srgb(0.2, 0.2, 0.3)
        } else {
            Color::srgb(0.1, 0.1, 0.1)
        }),
        TalentNode { 
            talent_id: talent.id.clone(),
            tree_type: *tree_type,
        },
    )).with_children(|node| {
        // Talent name and rank
        node.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new(&talent.name),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
            header.spawn((
                Text::new(format!("{}/{}", current_rank, talent.max_ranks)),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(1.0, 0.843, 0.0)),
            ));
        });
        
        // Description
        node.spawn((
            Text::new(&talent.description),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
        
        // Cost
        if current_rank < talent.max_ranks {
            node.spawn((
                Text::new(format!("Cost: {} points", talent.cost_per_rank)),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.5, 0.8, 1.0)),
            ));
        }
    });
}
