use crate::{game_backend, game_map, game_ui};

use bevy::prelude::*;

pub struct GameFrontendPlugin;

impl Plugin for GameFrontendPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .add_system(setup)
            .add_system(handle_movement)
            .add_system(show_cg)
            .add_system(camera_follow)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(bevy::time::FixedTimestep::step(
                        NPC_ANIMATION_INTERVAL as f64,
                    ))
                    .with_system(update_npc_frames),
            );
    }
}

fn show_cg(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut cgs: ResMut<game_backend::Cgs>,
    mut game_state: ResMut<game_backend::GameState>,
    mut cg_query: Query<(&mut Handle<Image>, &CgComponent, Entity), With<CgComponent>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let cg_id = match game_state.game_progress {
        game_backend::GameProgress::Intro => "intro",
        _ => return,
    };
    if camera_query.is_empty() {
        return;
    }
    let mut camera = camera_query.single_mut();
    camera.translation.x = 0.0;
    camera.translation.y = 0.0;

    game_state.is_showing_cg = true;

    if !keyboard_input.any_just_pressed([KeyCode::Return, KeyCode::Space]) {
        return;
    }

    let mut completed = false;
    for (mut texture, CgComponent(id), entity) in cg_query.iter_mut() {
        if id == cg_id {
            let cg = cgs.cgs.get_mut(id).unwrap();
            cg.index += 1;
            if cg.index >= cg.images.len() {
                commands.entity(entity).despawn();
                completed = true;
            } else {
                *texture = cg.images[cg.index].to_owned();
            }
        }
    }

    if completed {
        game_state.is_showing_cg = false;
        game_state.game_progress = match game_state.game_progress {
            game_backend::GameProgress::Intro => game_backend::GameProgress::Tutorial,
            _ => return,
        };
    }
}

fn update_npc_frames(
    mut commands: Commands,
    mut npcs: ResMut<game_backend::Npcs>,
    mut npc_query: Query<(&mut Handle<Image>, &NpcComponent, Entity), With<NpcComponent>>,
) {
    npcs.frame_count += 1;
    for (mut texture, NpcComponent(id), entity) in npc_query.iter_mut() {
        match npcs.npcs.get(id) {
            Some(npc) => {
                let frames = &npc.animation_frames;
                *texture = frames[npcs.frame_count % frames.len()].to_owned();
            }
            None => {
                commands.entity(entity).despawn();
            }
        }
    }
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    map: Res<game_map::Map>,
    tileset: Res<game_map::MapTileset>,
    npcs: Res<game_backend::Npcs>,
    cgs: Res<game_backend::Cgs>,
    mut game_state: ResMut<game_backend::GameState>,
    mut player_state: ResMut<PlayerState>,
) {
    if !map.loaded || !npcs.loaded || player_state.loaded {
        return;
    }
    player_state.x_pos = (map.start_pos.0 as f32 + 0.5) * TILE_WIDTH;
    player_state.y_pos = (map.start_pos.1 as f32 + 0.5) * TILE_HEIGHT;
    for direction in 0..4 {
        for frame in 0..PLAYER_ANIMATION_FRAMES {
            player_state.textures[direction][frame] =
                asset_server.load(format!("chars/mc/mc{}-{}.png", direction, frame));
        }
    }
    game_state.player_x = map.start_pos.0;
    game_state.player_y = map.start_pos.1;

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(player_state.x_pos, player_state.y_pos, PLAYER_Z),
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
                ..default()
            },
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(PLAYER_CENTER_X, PLAYER_CENTER_Y)),
                ..default()
            },
            texture: player_state.textures[0][0].to_owned(),
            ..default()
        },
        Protagonist,
    ));
    player_state.loaded = true;

    for x in 0..map.width {
        for y in 0..map.height {
            let handle = tileset.0.get(&map.tiles[x][y].tile_type).unwrap();
            commands.spawn((SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * game_map::Tile::WIDTH,
                        y as f32 * game_map::Tile::HEIGHT,
                        game_map::Tile::Z_LAYER,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.5, -0.5)),
                    ..default()
                },
                texture: handle.to_owned(),
                ..default()
            },));
        }
    }

    for (id, npc) in npcs.npcs.iter() {
        let handle = &npc.animation_frames[0];
        let (x, y) = npc.location;
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 + 0.5) * game_map::Tile::WIDTH,
                        (y as f32 + 0.5) * game_map::Tile::HEIGHT,
                        NPC_Z,
                    ),
                    scale: Vec3::new(NPC_SCALE, NPC_SCALE, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(NPC_CENTER_X, NPC_CENTER_Y)),
                    ..default()
                },
                texture: handle.to_owned(),
                ..default()
            },
            NpcComponent(id.to_owned()),
        ));
    }

    for (id, cg) in cgs.cgs.iter() {
        let handle = &cg.images[0];
        let scale_x = windows.get_primary().unwrap().width() / CG_WIDTH;
        let scale_y = windows.get_primary().unwrap().height() / CG_HEIGHT;
        let scale = if scale_x > scale_y { scale_x } else { scale_y };
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, CG_Z),
                    scale: Vec3::new(scale, scale, 1.0),
                    ..default()
                },
                texture: handle.to_owned(),
                ..default()
            },
            CgComponent(id.to_owned()),
        ));
    }
}

fn camera_follow(
    player_state: ResMut<PlayerState>,
    game_state: Res<game_backend::GameState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if !player_state.loaded || camera_query.is_empty() || game_state.is_showing_cg {
        return;
    }

    let mut camera = camera_query.single_mut();
    camera.translation.x = player_state.x_pos;
    camera.translation.y = player_state.y_pos;
}

fn handle_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<game_map::Map>,
    ui_state: Res<game_ui::UiState>,
    mut game_state: ResMut<game_backend::GameState>,
    mut player_state: ResMut<PlayerState>,
    mut player_query: Query<(&mut Transform, &mut Handle<Image>), With<Protagonist>>,
) {
    if !player_state.loaded
        || player_query.is_empty()
        || ui_state.is_textbox_focused
        || game_state.in_battle
        || game_state.is_showing_cg
    {
        return;
    }

    let mut delta_x = 0.0;
    let mut delta_y = 0.0;
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        delta_x -= time.delta_seconds() * PLAYER_VELOCITY;
        player_state.direction = 3;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        delta_x += time.delta_seconds() * PLAYER_VELOCITY;
        player_state.direction = 1;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        delta_y -= time.delta_seconds() * PLAYER_VELOCITY;
        player_state.direction = 0;
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        delta_y += time.delta_seconds() * PLAYER_VELOCITY;
        player_state.direction = 2;
    }

    let mut player = player_query.single_mut();

    if delta_x == 0.0 && delta_y == 0.0 {
        *player.1 = player_state.textures[player_state.direction][0].to_owned();
        return;
    }

    let is_valid = |x: f32, y: f32| {
        let tile_x = (x / TILE_WIDTH).floor() as usize;
        let tile_y = (y / TILE_HEIGHT).floor() as usize;
        let offset_x = x - tile_x as f32 * TILE_WIDTH;
        let offset_y = y - tile_y as f32 * TILE_HEIGHT;
        map.tiles[tile_x][tile_y].is_valid(offset_x, offset_y)
    };

    let new_x = if is_valid(player_state.x_pos + delta_x, player_state.y_pos) {
        player_state.x_pos + delta_x
    } else {
        player_state.x_pos
    };
    let new_y = if is_valid(player_state.x_pos, player_state.y_pos + delta_y) {
        player_state.y_pos + delta_y
    } else {
        player_state.y_pos
    };

    player.0.translation.x = new_x - PLAYER_SCALE * PLAYER_CENTER_X;
    player.0.translation.y = new_y - PLAYER_SCALE * PLAYER_CENTER_Y;
    player_state.x_pos = new_x;
    player_state.y_pos = new_y;
    game_state.player_x = (new_x / TILE_WIDTH).floor() as usize;
    game_state.player_y = (new_y / TILE_HEIGHT).floor() as usize;

    let animation_frame = (time.elapsed_seconds() / PLAYER_ANIMATION_INTERVAL).floor() as usize;
    *player.1 = player_state.textures[player_state.direction]
        [animation_frame % PLAYER_ANIMATION_FRAMES]
        .to_owned();
}

#[derive(Component)]
struct Protagonist;

#[derive(Component)]
struct NpcComponent(String);

#[derive(Component)]
struct CgComponent(String);

#[derive(Resource, Default)]
struct PlayerState {
    loaded: bool,
    x_pos: f32,
    y_pos: f32,
    direction: usize,
    textures: [[Handle<Image>; PLAYER_ANIMATION_FRAMES]; 4],
}

mod constants {
    use crate::game_map;

    pub const PLAYER_WIDTH: f32 = 320.0;
    pub const PLAYER_HEIGHT: f32 = 480.0;
    pub const PLAYER_CENTER_X: f32 = -10.0 / PLAYER_WIDTH;
    pub const PLAYER_CENTER_Y: f32 = -190.0 / PLAYER_HEIGHT;
    pub const PLAYER_Z: f32 = 10.0;
    pub const PLAYER_SCALE: f32 = 0.3;
    pub const PLAYER_VELOCITY: f32 = 400.0;
    pub const PLAYER_ANIMATION_INTERVAL: f32 = 0.2;
    pub const PLAYER_ANIMATION_FRAMES: usize = 4;

    pub const TILE_WIDTH: f32 = game_map::Tile::WIDTH;
    pub const TILE_HEIGHT: f32 = game_map::Tile::HEIGHT;

    pub const NPC_CENTER_X: f32 = 0.0;
    pub const NPC_CENTER_Y: f32 = -0.4;
    pub const NPC_SCALE: f32 = 0.4;
    pub const NPC_Z: f32 = 15.0;
    pub const NPC_ANIMATION_INTERVAL: f32 = 0.3;

    pub const CG_WIDTH: f32 = 1280.0;
    pub const CG_HEIGHT: f32 = 720.0;
    pub const CG_Z: f32 = 20.0;
}
use constants::*;
