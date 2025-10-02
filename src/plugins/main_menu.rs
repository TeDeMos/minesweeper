use std::fmt;
use std::fmt::{Display, Formatter};

use bevy::prelude::*;

use crate::AppState;
use crate::plugins::{Board, TextValSize};
use crate::utils::Nord;

#[derive(Component, Copy, Clone)]
enum MainMenuButton {
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl MainMenuButton {
    const ALL: [Self; 4] = [Self::Easy, Self::Medium, Self::Hard, Self::Extreme];

    fn get_color(self) -> Color {
        match self {
            MainMenuButton::Easy => Nord::GREEN,
            MainMenuButton::Medium => Nord::YELLOW,
            MainMenuButton::Hard => Nord::ORANGE,
            MainMenuButton::Extreme => Nord::RED,
        }
    }

    fn get_board(self) -> Board {
        match self {
            Self::Easy => Board::new(16, 12, 20),
            Self::Medium => Board::new(25, 20, 75),
            Self::Hard => Board::new(35, 27, 190),
            Self::Extreme => Board::new(55, 36, 500),
        }
    }
}

impl Display for MainMenuButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            MainMenuButton::Easy => "Easy",
            MainMenuButton::Medium => "Medium",
            MainMenuButton::Hard => "Hard",
            MainMenuButton::Extreme => "Extreme",
        })
    }
}

#[derive(Component)]
struct MainMenuRoot;

fn spawn(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Nord::NIGHT[0]),
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::VMin(100.0),
                    height: Val::VMin(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::vertical(Val::Percent(5.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Minesweeper"),
                        TextColor(Nord::SNOW[2]),
                        TextValSize(Val::Percent(12.0)),
                    ));
                    for button in MainMenuButton::ALL {
                        parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Percent(65.0),
                                    height: Val::Percent(13.0),
                                    justify_content: JustifyContent::Center,
                                    border: UiRect::all(Val::Percent(0.5)),
                                    ..default()
                                },
                                BackgroundColor(Nord::NIGHT[0]),
                                BorderColor(Nord::FROST[3]),
                                BorderRadius::all(Val::Percent(5.0)),
                                button,
                            ))
                            .with_child((
                                Text::new(button.to_string()),
                                TextColor(button.get_color()),
                                TextValSize(Val::Percent(75.0)),
                            ));
                    }
                });
        });
}

fn buttons_interaction(
    mut commands: Commands, mut next_state: ResMut<NextState<AppState>>,
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background, mut border, &button) in &mut interactions {
        match interaction {
            Interaction::Pressed => {
                commands.insert_resource(button.get_board());
                next_state.set(AppState::Playing);
            },
            Interaction::Hovered => {
                *background = BackgroundColor(Nord::NIGHT[1]);
                *border = BorderColor(Nord::FROST[1]);
            },
            Interaction::None => {
                *background = BackgroundColor(Nord::NIGHT[0]);
                *border = BorderColor(Nord::FROST[3]);
            },
        }
    }
}

fn despawn(mut commands: Commands, root: Single<Entity, With<MainMenuRoot>>) {
    commands.entity(root.into_inner()).despawn();
}

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), spawn)
        .add_systems(Update, buttons_interaction.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), despawn);
}
