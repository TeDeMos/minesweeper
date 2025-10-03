use std::fmt;
use std::fmt::{Display, Formatter};

use bevy::ecs::component::Mutable;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use crate::AppState;
use crate::plugins::{Board, TextValSize};
use crate::utils::Nord;

#[derive(Component, Clone)]
struct MenuButton;

trait Cycling: Display + Copy + Component<Mutability = Mutable> {
    fn next(self) -> Self;
    fn color(self) -> Color;
    fn label(self) -> String;
}

#[derive(Component)]
struct TargetText;

#[derive(Component, Copy, Clone)]
pub enum Size {
    Small,
    Medium,
    Big,
    Huge,
}

impl Size {
    pub fn dimensions(self) -> (usize, usize) {
        match self {
            Size::Small => (16, 9),
            Size::Medium => (32, 18),
            Size::Big => (48, 27),
            Size::Huge => (64, 36),
        }
    }
}

impl Cycling for Size {
    fn next(self) -> Self {
        match self {
            Size::Small => Size::Medium,
            Size::Medium => Size::Big,
            Size::Big => Size::Huge,
            Size::Huge => Size::Small,
        }
    }

    fn color(self) -> Color {
        match self {
            Size::Small => Nord::GREEN,
            Size::Medium => Nord::YELLOW,
            Size::Big => Nord::ORANGE,
            Size::Huge => Nord::RED,
        }
    }

    fn label(self) -> String { String::from("Size:") }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Size::Small => "Small",
            Size::Medium => "Medium",
            Size::Big => "Big",
            Size::Huge => "Huge",
        })
    }
}

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
    Debug,
}

impl Difficulty {
    pub fn mine_count(self, cells: usize) -> usize {
        match self {
            Difficulty::Easy => cells / 10,
            Difficulty::Medium => cells * 3 / 20,
            Difficulty::Hard => cells / 5,
            Difficulty::Extreme => cells / 4,
            Difficulty::Debug => 36,
        }
    }

    pub fn is_debug(self) -> bool { self == Self::Debug }
}

impl Cycling for Difficulty {
    fn next(self) -> Self {
        match self {
            Difficulty::Easy => Self::Medium,
            Difficulty::Medium => Self::Hard,
            Difficulty::Hard => Self::Extreme,
            Difficulty::Extreme => Self::Debug,
            Difficulty::Debug => Self::Easy,
        }
    }

    fn color(self) -> Color {
        match self {
            Difficulty::Easy => Nord::GREEN,
            Difficulty::Medium => Nord::YELLOW,
            Difficulty::Hard => Nord::ORANGE,
            Difficulty::Extreme => Nord::RED,
            Difficulty::Debug => Nord::PURPLE,
        }
    }

    fn label(self) -> String { String::from("Difficulty:") }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Extreme => "Extreme",
            Difficulty::Debug => "Debug",
        })
    }
}

#[derive(Component)]
struct Begin;

#[derive(Component)]
struct MainMenuRoot;

fn button_base<M: Bundle>(marker: M) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(14.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Percent(0.5)),
            column_gap: Val::Percent(5.0),
            ..default()
        },
        BackgroundColor(Nord::NIGHT[0]),
        BorderColor(Nord::FROST[3]),
        BorderRadius::all(Val::Percent(5.0)),
        MenuButton,
        marker,
    )
}

fn cycling_button<C: Cycling>(parent: &mut RelatedSpawnerCommands<ChildOf>, cycling: C) {
    parent.spawn(button_base(cycling)).with_children(|parent| {
        parent.spawn((
            Text::new(cycling.label()),
            TextColor(Nord::SNOW[2]),
            TextValSize(Val::Percent(45.0)),
        ));
        parent.spawn((
            Text::new(cycling.to_string()),
            TextColor(cycling.color()),
            TextValSize(Val::Percent(45.0)),
            TargetText,
        ));
    });
}

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
                    cycling_button(parent, Size::Small);
                    cycling_button(parent, Difficulty::Easy);
                    parent.spawn(button_base(Begin)).with_child((
                        Text::new("Begin"),
                        TextColor(Nord::SNOW[2]),
                        TextValSize(Val::Percent(45.0)),
                    ));
                });
        });
}

fn buttons_hover(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut background, mut border) in &mut interactions {
        match interaction {
            Interaction::Hovered => {
                *background = BackgroundColor(Nord::NIGHT[1]);
                *border = BorderColor(Nord::FROST[1]);
            },
            Interaction::None => {
                *background = BackgroundColor(Nord::NIGHT[0]);
                *border = BorderColor(Nord::FROST[3]);
            },
            Interaction::Pressed => {},
        }
    }
}

fn cycling_click<C: Cycling>(
    interaction: Single<
        (&Interaction, &mut C, &Children),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut text: Query<(&mut Text, &mut TextColor), With<TargetText>>,
) {
    let (Interaction::Pressed, mut cycling, children) = interaction.into_inner() else {
        return;
    };
    *cycling = cycling.next();
    for &e in children {
        let Ok((mut text, mut color)) = text.get_mut(e) else { continue };
        text.0 = cycling.to_string();
        color.0 = cycling.color();
    }
}

fn begin_click(
    mut commands: Commands, mut next_state: ResMut<NextState<AppState>>,
    interaction: Single<&Interaction, (Changed<Interaction>, With<MenuButton>, With<Begin>)>,
    size: Single<&Size, With<MenuButton>>, difficulty: Single<&Difficulty, With<MenuButton>>,
) {
    if **interaction != Interaction::Pressed {
        return;
    }
    commands.insert_resource(Board::new(**size, **difficulty));
    next_state.set(AppState::Playing);
}

fn despawn(mut commands: Commands, root: Single<Entity, With<MainMenuRoot>>) {
    commands.entity(root.into_inner()).despawn();
}

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), spawn)
        .add_systems(
            Update,
            (buttons_hover, cycling_click::<Size>, cycling_click::<Difficulty>, begin_click)
                .run_if(in_state(AppState::Menu)),
        )
        .add_systems(OnExit(AppState::Menu), despawn);
}
