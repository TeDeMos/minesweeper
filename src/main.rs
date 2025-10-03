#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap,
    clippy::cast_sign_loss, clippy::needless_pass_by_value, clippy::type_complexity
)]

use bevy::prelude::*;
use plugins::{
    board, camera, game_assets, hide_children_on_hover, hud, main_menu, mouse, text_val_size,
};

mod plugins;
mod utils;

#[derive(States, Copy, Clone, PartialEq, Eq, Debug, Hash, Default)]
enum AppState {
    #[default]
    Menu,
    Playing,
    Won,
    Lost,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            board,
            camera,
            game_assets,
            hide_children_on_hover,
            hud,
            main_menu,
            mouse,
            text_val_size,
        ))
        .init_state::<AppState>()
        .run();
}
