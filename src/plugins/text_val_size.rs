use bevy::prelude::*;

#[derive(Component)]
pub struct TextValSize(pub Val);

fn update(
    texts: Query<(&TextValSize, &mut TextFont, &ChildOf), With<Text>>,
    nodes: Query<&ComputedNode, Changed<ComputedNode>>,
) {
    for (val, mut font, child_of) in texts {
        let Ok(parent) = nodes.get(child_of.parent()) else { continue };
        let Ok(size) = val.0.resolve(1.0, parent.size.y, parent.size) else { continue };
        font.font_size = size;
    }
}

pub fn text_val_size(app: &mut App) { app.add_systems(Update, update); }
