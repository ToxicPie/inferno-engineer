use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>().add_system(game_loop);
    }
}

fn game_loop() {}

#[derive(Resource)]
pub struct GameState {
    pub player_level: i32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { player_level: 0 }
    }
}
