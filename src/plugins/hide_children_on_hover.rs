use bevy::prelude::*;

#[derive(Component)]
#[require(Interaction)]
pub struct HideChildrenOnHover;

fn update(
    interaction: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<HideChildrenOnHover>),
    >,
    mut visibility: Query<&mut Visibility>,
) {
    for (i, c) in interaction {
        let desired = match i {
            Interaction::Hovered => Visibility::Hidden,
            Interaction::None => Visibility::Inherited,
            Interaction::Pressed => continue,
        };
        for &e in c {
            *visibility.get_mut(e).unwrap() = desired;
        }
    }
}

pub fn hide_children_on_hover(app: &mut App) { app.add_systems(Update, update); }
