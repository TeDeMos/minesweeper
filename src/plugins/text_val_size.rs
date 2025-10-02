use bevy::prelude::*;

#[derive(Component)]
pub struct TextValSize(pub Val);

fn update(
    texts: Query<(&TextValSize, &mut TextFont, &ChildOf), With<Text>>,
    nodes: Query<&ComputedNode, Changed<ComputedNode>>,
) {
    let viewport = Vec2::splat(0.0);
    for (val, mut font, child_of) in texts {
        let Ok(parent) = nodes.get(child_of.parent()) else { continue };
        let Ok(size) = val.0.resolve(parent.size.y, viewport) else { continue };
        font.font_size = size;
    }
}

pub fn text_val_size(app: &mut App) { app.add_systems(Update, update); }
