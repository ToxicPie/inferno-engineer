use crate::{commands, npcs};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;
use std::collections::HashMap;

pub struct GameBackendPlugin;

impl Plugin for GameBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<NpcFile>::new(&["npcs.ron"]))
            .add_plugin(RonAssetPlugin::<CgFile>::new(&["cgs.ron"]))
            .init_resource::<GameState>()
            .add_event::<CommandExecutionEvent>()
            .add_event::<CommandResultEvent>()
            .add_event::<NpcActionEvent>()
            .add_event::<NpcResponseEvent>()
            .init_resource::<NpcFileHandle>()
            .init_resource::<Npcs>()
            .init_resource::<ActiveNpc>()
            .init_resource::<CgFileHandle>()
            .init_resource::<Cgs>()
            .add_startup_system(load_files)
            .add_system(prepare_npcs)
            .add_system(prepare_cgs)
            .add_system(game_loop);
    }
}

fn load_files(
    asset_server: Res<AssetServer>,
    mut npc_handle: ResMut<NpcFileHandle>,
    mut cg_handle: ResMut<CgFileHandle>,
) {
    npc_handle.0 = asset_server.load("npcfile.npcs.ron");
    cg_handle.0 = asset_server.load("cgfile.cgs.ron");
}

fn prepare_npcs(
    asset_server: Res<AssetServer>,
    mut npc_list: ResMut<Npcs>,
    npc_handle: ResMut<NpcFileHandle>,
    npc_file: Res<Assets<NpcFile>>,
) {
    if npc_list.loaded {
        return;
    }
    let Some(npc_file) = npc_file.get(&npc_handle.0) else { return; };

    for (id, (x, y, frames)) in npc_file.npcs.iter() {
        let npc = Npc {
            animation_frames: (0..*frames)
                .map(|frame| asset_server.load(format!("chars/{}/{}-{}.png", id, id, frame)))
                .collect(),
            location: (*x, *y),
        };
        npc_list.npcs.insert(id.to_owned(), npc);
    }
    npc_list.loaded = true;
}

fn prepare_cgs(
    asset_server: Res<AssetServer>,
    mut cg_list: ResMut<Cgs>,
    cg_handle: ResMut<CgFileHandle>,
    cg_file: Res<Assets<CgFile>>,
) {
    if cg_list.loaded {
        return;
    }
    let Some(cg_file) = cg_file.get(&cg_handle.0) else { return; };

    for (id, frames) in cg_file.cgs.iter() {
        let cg = Cg {
            images: (0..*frames)
                .map(|frame| asset_server.load(format!("cgs/{}-{}.png", id, frame)))
                .collect(),
            index: 0,
        };
        cg_list.cgs.insert(id.to_owned(), cg);
    }
    cg_list.loaded = true;
}

fn game_loop(
    mut game_state: ResMut<GameState>,
    mut active_npc: ResMut<ActiveNpc>,
    mut npc_state: ResMut<Npcs>,
    mut execution_events: EventReader<CommandExecutionEvent>,
    mut action_events: EventReader<NpcActionEvent>,
    mut result_events: EventWriter<CommandResultEvent>,
    mut response_events: EventWriter<NpcResponseEvent>,
) {
    // handle commands
    for execution in execution_events.iter() {
        let CommandExecutionEvent(command) = execution;
        let message = match commands::execute_command(&mut game_state, command.as_str()) {
            Ok(msg) => msg,
            Err(msg) => format!("Error: {}", msg),
        };
        result_events.send(CommandResultEvent(message));
    }

    // handle npc encounter
    if active_npc.0.is_none() {
        for (id, npc) in npc_state.npcs.iter() {
            if npc.location == (game_state.player_x, game_state.player_y) {
                active_npc.0 = Some(npcs::get_npc_by_id(id).unwrap());
                active_npc
                    .0
                    .as_mut()
                    .unwrap()
                    .handle_action(&npcs::PlayerAction::Ping, &mut game_state);
                game_state.in_battle = true;
                break;
            }
        }
    } else {
        let current_npc = active_npc.0.as_mut().unwrap();
        for action in action_events.iter() {
            current_npc.handle_action(&action.0, &mut game_state);
        }

        let action_queue = game_state.action_queue.clone();
        game_state.action_queue.clear();
        for action in action_queue.iter() {
            current_npc.handle_action(action, &mut game_state);
        }

        if let Some(response) = current_npc.get_response() {
            response_events.send(NpcResponseEvent(response));
        }

        if current_npc.job_completed() {
            npc_state.npcs.remove(current_npc.id());
            game_state.in_battle = false;
            active_npc.0 = None;
        }
    }
}

#[derive(Resource)]
pub struct GameState {
    pub game_progress: GameProgress,
    pub is_showing_cg: bool,
    pub player_level: i32,
    pub player_hitpoints: i32,
    pub player_max_hp: i32,
    pub player_atk: i32,
    pub player_def: i32,
    pub player_x: usize,
    pub player_y: usize,
    pub in_battle: bool,
    pub action_queue: Vec<npcs::PlayerAction>,
}

#[derive(Resource, Default)]
pub struct ActiveNpc(pub Option<Box<dyn npcs::Npc>>);

#[derive(Resource, Default)]
pub struct Npc {
    pub animation_frames: Vec<Handle<Image>>,
    pub location: (usize, usize),
}

#[derive(Resource, Default)]
pub struct Npcs {
    pub npcs: HashMap<String, Npc>,
    pub frame_count: usize,
    pub loaded: bool,
}

#[derive(Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "59c820f3-107f-4f40-b183-39f1b2cab9cd"]
struct NpcFile {
    npcs: HashMap<String, (usize, usize, usize)>,
}

#[derive(Resource, Default)]
pub struct Cg {
    pub images: Vec<Handle<Image>>,
    pub index: usize,
}

#[derive(Resource, Default)]
struct NpcFileHandle(Handle<NpcFile>);

#[derive(Resource, Default)]
pub struct Cgs {
    pub cgs: HashMap<String, Cg>,
    pub loaded: bool,
}

#[derive(Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "bbd0c69f-3845-423a-9fae-8f8a107ec2b5"]
struct CgFile {
    cgs: HashMap<String, usize>,
}

#[derive(Resource, Default)]
struct CgFileHandle(Handle<CgFile>);

pub struct CommandExecutionEvent(pub String);

pub struct CommandResultEvent(pub String);

#[derive(Default)]
pub struct NpcActionEvent(pub npcs::PlayerAction);

pub struct NpcResponseEvent(pub npcs::NpcResponse);

impl Default for GameState {
    fn default() -> Self {
        GameState {
            game_progress: GameProgress::Intro,
            is_showing_cg: false,
            player_level: 0,
            player_hitpoints: 20,
            player_max_hp: 20,
            player_atk: 5,
            player_def: 2,
            player_x: 0,
            player_y: 0,
            in_battle: false,
            action_queue: vec![],
        }
    }
}

impl GameState {
    pub fn player_details(&self) -> String {
        let mut res = String::new();
        res.push_str(format!("Your access level: {}\n", self.player_level).as_str());
        res.push_str(format!("HP: {} / {}\n", self.player_hitpoints, self.player_max_hp).as_str());
        res.push_str(format!("ATK: {}\n", self.player_atk).as_str());
        res.push_str(format!("DEF: {}", self.player_def).as_str());
        res
    }
}

pub enum GameProgress {
    Intro,
    Tutorial,
    HasPanel,
    HasTerminal,
}
