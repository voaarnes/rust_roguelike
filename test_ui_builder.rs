use bevy::prelude::*;

// Test what UI building types are available in Bevy 0.16
fn test_ui_builder(mut commands: Commands) {
    commands.spawn(Node::default())
        .with_children(|parent| {
            // Test what 'parent' is - this should tell us the type
            // parent should be of type ChildBuilder or similar
            parent.spawn(Node::default());
        });
}

fn main() {}
