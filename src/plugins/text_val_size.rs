use bevy::prelude::*;

#[derive(Component)]
pub struct TextValSize(pub Val);

fn update(
    texts: Query<(&TextValSize, &mut TextFont, &ChildOf, Option<&Children>), With<Text>>,
    nodes: Query<&ComputedNode, Changed<ComputedNode>>,
    mut spans: Query<&mut TextFont, (With<TextSpan>, Without<TextValSize>)>
) {
    for (val, mut font, child_of, children) in texts {
        let Ok(parent) = nodes.get(child_of.parent()) else { continue };
        let Ok(size) = val.0.resolve(1.0, parent.size.y, parent.size) else { continue };
        font.font_size = size;
        for &e in children.into_iter().flatten() {
            let Ok(mut font) = spans.get_mut(e) else { continue };
            font.font_size = size;
        }
    }
}

pub fn text_val_size(app: &mut App) { app.add_systems(Update, update); }
