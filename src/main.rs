mod canvas;
mod commands;
mod game_backend;
mod game_frontend;
mod game_map;
mod game_ui;
mod npcs;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(game_backend::GameBackendPlugin)
        .add_plugin(game_map::MapPlugin)
        .add_plugin(game_frontend::GameFrontendPlugin)
        .add_plugin(game_ui::GameUiPlugin)
        .run();
}
