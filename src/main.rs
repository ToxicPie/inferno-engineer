mod canvas;
mod commands;
mod game_ui;
mod gameplay;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_plugin(game_ui::GameUiPlugin)
        .run();
}
