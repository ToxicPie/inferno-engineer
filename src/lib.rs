mod canvas;
mod commands;
mod game_ui;
mod gameplay;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(canvas::CanvasPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_plugin(game_ui::GameUiPlugin)
        .run();
}
