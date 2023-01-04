mod alice;

use crate::game_backend;

pub trait Npc: Sync + Send {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn info(&self) -> String;
    fn handle_action(
        &mut self,
        action: &PlayerAction,
        game_state: &mut game_backend::GameState,
    ) -> ();
    fn get_response(&mut self) -> Option<NpcResponse>;
    fn job_completed(&self) -> bool;
}

pub fn get_npc_by_id(name: &str) -> Option<Box<dyn Npc>> {
    let name = name.trim();
    match name.to_lowercase().as_str() {
        "alice" => Some(Box::new(alice::AliceNpc::default())),
        _ => None,
    }
}

#[derive(Clone, Copy, Default)]
pub enum PlayerAction {
    #[default]
    Ping,
    Respond(usize),
    Attack(i32),
}

#[derive(Clone)]
pub struct NpcResponse {
    pub message: String,
    pub name: Option<String>,
    pub choices: Vec<String>,
}

#[macro_export]
macro_rules! npc_response {
    ($msg:expr) => {
        npcs::NpcResponse {
            message: String::from($msg),
            name: None,
            choices: vec![],
        }
    };
    ($msg:expr, $name:expr) => {
        npcs::NpcResponse {
            message: String::from($msg),
            name: Some(String::from($name)),
            choices: vec![],
        }
    };
    ($msg:expr; $($resp:expr),+ $(,)?) => {
        npcs::NpcResponse {
            message: String::from($msg),
            name: None,
            choices: vec![$($resp.to_string()),+],
        }
    };
    ($msg:expr, $name:expr; $($resp:expr),+ $(,)?) => {
        npcs::NpcResponse {
            message: String::from($msg),
            name: Some(String::from($name)),
            choices: vec![$($resp.to_string()),+],
        }
    };
}
