use std::array;

use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub bomb: Handle<Image>,
    pub bomb_clicked: Handle<Image>,
    pub covered: Handle<Image>,
    pub empty: Handle<Image>,
    pub flagged: Handle<Image>,
    pub neighbours: [Handle<Image>; 8],
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bomb: asset_server.load("bomb.png"),
        bomb_clicked: asset_server.load("bomb_clicked.png"),
        covered: asset_server.load("covered.png"),
        empty: asset_server.load("empty.png"),
        flagged: asset_server.load("flagged.png"),
        neighbours: array::from_fn(|i| asset_server.load(format!("{}.png", i + 1))),
    });
}

pub fn game_assets(app: &mut App) {
    app.add_systems(Startup, spawn);
}