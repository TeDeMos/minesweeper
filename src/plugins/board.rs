use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;

use crate::AppState;
use crate::plugins::{LeftClicked, MineCount, RightClicked};

const OFFSETS: [(isize, isize); 8] =
    [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

#[derive(Resource)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub tiles: Box<[Box<[Entity]>]>,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self { width, height, mines, tiles: Box::new([]) }
    }

    pub fn size(&self) -> Vec2 { Vec2::new(self.width as _, self.height as _) }

    pub fn get_from_world(&self, v: Vec2) -> Option<Entity> {
        if v.x >= 0.0 && v.y <= 0.0 {
            self.tiles.get(v.x as usize).and_then(|c| c.get(-v.y as usize)).copied()
        } else {
            None
        }
    }

    fn get_neighbours(&self, coordinates: Coordinates) -> impl Iterator<Item = Entity> {
        OFFSETS.into_iter().filter_map(move |(dx, dy)| {
            self.tiles
                .get(coordinates.x.checked_add_signed(dx)?)?
                .get(coordinates.y.checked_add_signed(dy)?)
                .copied()
        })
    }
}

fn initialize(mut board: ResMut<Board>, asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut chosen = 0;
    let mut temp = vec![vec![0; board.height]; board.width];
    let mut rand = rand::rng();
    while chosen < board.mines {
        let x = rand.random_range(0..board.width);
        let y = rand.random_range(0..board.height);
        if temp[x][y] >= 0 {
            temp[x][y] = -1;
            for (xd, yd) in OFFSETS {
                if let Some(x) = x.checked_add_signed(xd)
                    && let Some(c) = temp.get_mut(x)
                    && let Some(y) = y.checked_add_signed(yd)
                    && let Some(n) = c.get_mut(y)
                    && *n != -1
                {
                    *n += 1;
                }
            }
            chosen += 1;
        }
    }
    board.tiles = temp
        .into_iter()
        .enumerate()
        .map(|(x, c)| {
            c.into_iter()
                .enumerate()
                .map(|(y, n)| Tile::spawn(x, y, n, &mut commands, &asset_server))
                .collect()
        })
        .collect();
}

#[derive(Component)]
#[require(Sprite, Coordinates, TileState, TileValue)]
struct Tile;

impl Tile {
    fn spawn(
        x: usize, y: usize, bombs: i8, commands: &mut Commands, asset_server: &AssetServer,
    ) -> Entity {
        commands
            .spawn((
                Tile,
                Sprite {
                    image: asset_server.load("covered.png"),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    anchor: Anchor::TopLeft,
                    image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
                    ..default()
                },
                Transform::from_translation(Vec3::new(x as f32, -(y as f32), 0.0)),
                Coordinates::new(x, y),
                TileValue::from_i8(bombs),
            ))
            .id()
    }
}

#[derive(Component, Default, Copy, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
enum TileState {
    #[default]
    Covered,
    Flagged,
    Uncovered,
}

#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
enum TileValue {
    #[default]
    Empty,
    Neighbours(u8),
    Bomb,
}

impl TileValue {
    fn get_image(self, asset_server: &AssetServer) -> Handle<Image> {
        match self {
            Self::Empty => asset_server.load("empty.png"),
            Self::Neighbours(n) => asset_server.load(format!("{n}.png")),
            Self::Bomb => asset_server.load("bomb_clicked.png"),
        }
    }

    fn from_i8(v: i8) -> Self {
        match v {
            -1 => Self::Bomb,
            0 => Self::Empty,
            1..=8 => Self::Neighbours(v as _),
            _ => panic!("Unexpected value"),
        }
    }
}

#[derive(Component)]
struct Flood;

fn uncover(
    sprite: &mut Sprite, state: &mut TileState, value: TileValue, coordinates: Coordinates,
    commands: &mut Commands, asset_server: &AssetServer, board: &Board,
) {
    sprite.image = value.get_image(asset_server);
    *state = TileState::Uncovered;
    match value {
        TileValue::Empty => mark_neighbours(commands, board, coordinates),
        TileValue::Bomb => commands.set_state(AppState::Lost),
        TileValue::Neighbours(_) => {},
    }
}

fn mark_neighbours(commands: &mut Commands, board: &Board, coordinates: Coordinates) {
    for entity in board.get_neighbours(coordinates) {
        commands.entity(entity).insert(Flood);
    }
}

fn left_click(
    mut commands: Commands,
    clicked: Query<
        (Entity, &mut Sprite, &mut TileState, &TileValue, &Coordinates),
        (With<Tile>, With<LeftClicked>),
    >,
    tiles: Query<&TileState, Without<LeftClicked>>, board: Res<Board>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut sprite, mut state, &value, &coordinates) in clicked {
        match *state {
            TileState::Covered => {
                uncover(
                    &mut sprite, &mut state, value, coordinates, &mut commands, &asset_server,
                    &board,
                );
            },
            TileState::Uncovered => match value {
                TileValue::Neighbours(n) => {
                    if board
                        .get_neighbours(coordinates)
                        .filter(|&e| *tiles.get(e).unwrap() == TileState::Flagged)
                        .count()
                        == n as usize
                    {
                        mark_neighbours(&mut commands, &board, coordinates);
                    }
                },
                TileValue::Empty | TileValue::Bomb => {},
            },
            TileState::Flagged => {},
        }
        commands.entity(entity).remove::<LeftClicked>();
    }
}

fn flood(
    mut commands: Commands,
    clicked: Query<
        (Entity, &mut Sprite, &mut TileState, &TileValue, &Coordinates),
        (With<Tile>, With<Flood>),
    >,
    board: Res<Board>, asset_server: Res<AssetServer>,
) {
    for (entity, mut sprite, mut state, &value, &coordinates) in clicked {
        if *state == TileState::Covered {
            uncover(
                &mut sprite, &mut state, value, coordinates, &mut commands, &asset_server, &board,
            );
        }
        commands.entity(entity).remove::<Flood>();
    }
}

fn right_click(
    mut commands: Commands,
    clicked: Query<(Entity, &mut Sprite, &mut TileState), (With<Tile>, With<RightClicked>)>,
    asset_server: Res<AssetServer>, mut count: ResMut<MineCount>,
) {
    for (entity, mut sprite, mut state) in clicked {
        match *state {
            TileState::Covered => {
                sprite.image = asset_server.load("flagged.png");
                *state = TileState::Flagged;
                count.0 -= 1;
            },
            TileState::Flagged => {
                sprite.image = asset_server.load("covered.png");
                *state = TileState::Covered;
                count.0 += 1;
            },
            TileState::Uncovered => {},
        }
        commands.entity(entity).remove::<RightClicked>();
    }
}

fn check_win(
    changed: Query<(), (Changed<TileState>, With<Tile>)>, tiles: Query<&TileState, With<Tile>>,
    board: Res<Board>, mut next_state: ResMut<NextState<AppState>>,
) {
    if !changed.is_empty()
        && tiles.into_iter().filter(|&&s| s != TileState::Uncovered).count() == board.mines
    {
        next_state.set(AppState::Won);
    }
}

fn add_flags(
    tiles: Query<(&mut Sprite, &TileState, &TileValue), With<Tile>>, asset_server: Res<AssetServer>,
) {
    for (mut sprite, &state, &value) in tiles {
        if state == TileState::Covered && value == TileValue::Bomb {
            sprite.image = asset_server.load("flagged.png");
        }
    }
}

fn uncover_bombs(
    tiles: Query<(&mut Sprite, &TileState, &TileValue), With<Tile>>, asset_server: Res<AssetServer>,
) {
    for (mut sprite, &state, &value) in tiles {
        if state == TileState::Covered && value == TileValue::Bomb {
            sprite.image = asset_server.load("bomb.png");
        }
    }
}

fn despawn(mut commands: Commands, board: Res<Board>) {
    for &e in board.tiles.iter().flatten() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<Board>();
}

pub fn board(app: &mut App) {
    app.add_systems(OnEnter(AppState::Playing), initialize)
        .add_systems(
            Update,
            (left_click, flood, right_click, check_win).run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnEnter(AppState::Won), add_flags)
        .add_systems(OnEnter(AppState::Lost), uncover_bombs)
        .add_systems(OnEnter(AppState::Menu), despawn.run_if(resource_exists::<Board>));
}
