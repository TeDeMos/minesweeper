use bevy::prelude::*;

use crate::AppState;
use crate::plugins::main_menu::Cycling;
use crate::plugins::{Board, HideChildrenOnHover, TextValSize};
use crate::utils::Nord;

#[derive(Component)]
struct MineText;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct Message;

#[derive(Component)]
struct HudRoot;

#[derive(Resource)]
pub struct MineCount(pub i32);

#[derive(Resource)]
struct Elapsed(f32);

fn spawn(mut commands: Commands, board: Res<Board>) {
    commands.insert_resource(MineCount(board.mines as _));
    commands.insert_resource(Elapsed(0.0));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::VMin(100.0),
                        height: Val::VMin(25.0),
                        padding: UiRect::new(
                            Val::VMin(10.0),
                            Val::VMin(10.0),
                            Val::VMin(1.0),
                            Val::VMin(19.0),
                        ),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    HideChildrenOnHover,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Nord::NIGHT[0]),
                            BorderRadius::all(Val::Percent(10.0)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(""),
                                TextValSize(Val::Percent(45.0)),
                                MineText,
                            ));
                            parent.spawn((
                                Text::new(""),
                                TextValSize(Val::Percent(45.0)),
                                TimeText,
                            ));
                            board.size.spawn(Val::Percent(45.0), false, parent);
                            board.difficulty.spawn(Val::Percent(45.0), false, parent);
                        });
                });
            parent
                .spawn((
                    Node {
                        width: Val::VMin(90.0),
                        height: Val::VMin(20.0),
                        padding: UiRect::new(
                            Val::VMin(10.0),
                            Val::VMin(10.0),
                            Val::VMin(13.0),
                            Val::VMin(1.0),
                        ),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Visibility::Hidden,
                    Message,
                    HideChildrenOnHover,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Nord::NIGHT[0]),
                            BorderRadius::all(Val::Percent(10.0)),
                        ))
                        .with_child((
                            Text::new("M: Menu, R: Restart"),
                            TextValSize(Val::Percent(45.0)),
                        ));
                });
        });
}

fn reset(
    board: Res<Board>, mut count: ResMut<MineCount>, mut elapsed: ResMut<Elapsed>,
    mut message: Single<&mut Visibility, With<Message>>,
) {
    count.0 = board.mines as _;
    elapsed.0 = 0.0;
    **message = Visibility::Hidden;
}

fn update_mines(count: Res<MineCount>, text: Single<&mut Text, With<MineText>>) {
    text.into_inner().0 = format!("Mines: {}", count.0);
}

fn update_time(
    mut text: Single<&mut Text, With<TimeText>>, time: Res<Time>, mut elapsed: ResMut<Elapsed>,
) {
    elapsed.0 += time.delta_secs();
    let rounded = elapsed.0 as u32;
    let h = rounded / 3600;
    let m = (rounded % 3600) / 60;
    let s = rounded % 60;
    text.0 = match (h, m, s) {
        (1.., ..) => format!("Time: {h}:{m:02}:{s:02}"),
        (0, 1.., ..) => format!("Time: {m}:{s:02}"),
        (0, 0, ..) => format!("Time: {s}"),
    };
}

fn force_zero(mut text: Single<&mut Text, With<MineText>>) { text.0 = String::from("Mines: 0"); }

fn show_message(mut message: Single<&mut Visibility, With<Message>>) {
    **message = Visibility::Visible;
}

fn despawn(mut commands: Commands, root: Single<Entity, With<HudRoot>>) {
    commands.entity(*root).despawn();
    commands.remove_resource::<MineCount>();
    commands.remove_resource::<Elapsed>();
}

fn wait_for_key(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::KeyM) {
        next_state.set(AppState::Menu);
    } else if input.just_pressed(KeyCode::KeyR) {
        next_state.set(AppState::Playing);
    }
}

pub fn hud(app: &mut App) {
    app.add_systems(OnExit(AppState::Menu), spawn)
        .add_systems(OnEnter(AppState::Playing), reset)
        .add_systems(
            Update,
            (update_mines.run_if(resource_exists_and_changed::<MineCount>), update_time)
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnExit(AppState::Playing), show_message)
        .add_systems(OnEnter(AppState::Won), force_zero)
        .add_systems(
            Update,
            wait_for_key.run_if(in_state(AppState::Won).or(in_state(AppState::Lost))),
        )
        .add_systems(OnEnter(AppState::Menu), despawn);
}
