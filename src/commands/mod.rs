mod commands;
mod fireball;
mod help;
mod man;

use crate::game_backend;

pub trait GameCommand {
    fn synopsis(&self) -> &'static str;
    fn man_page(&self) -> &'static str;
    fn required_level(&self) -> i32;
    fn execute(
        &self,
        game_state: &mut game_backend::GameState,
        argv: &[&str],
    ) -> Result<String, String>;
}

pub struct InvalidCommand;

impl GameCommand for InvalidCommand {
    fn synopsis(&self) -> &'static str {
        unimplemented!()
    }
    fn man_page(&self) -> &'static str {
        unimplemented!()
    }
    fn required_level(&self) -> i32 {
        i32::MIN
    }
    fn execute(
        &self,
        _game_state: &mut game_backend::GameState,
        argv: &[&str],
    ) -> Result<String, String> {
        Err(format!("Invalid command: {}", argv[0]))
    }
}

const COMMAND_LIST: [&'static str; 4] = ["commands", "help", "man", "fireball"];

pub fn get_command_by_name(name: &str) -> Option<Box<dyn GameCommand>> {
    let name = name.trim();
    match name.to_lowercase().as_str() {
        "commands" => Some(Box::new(commands::CommandsCommand)),
        "help" => Some(Box::new(help::HelpCommand)),
        "man" | "manual" => Some(Box::new(man::ManCommand)),
        "fireball" => Some(Box::new(fireball::FireballCommand)),
        _ => None,
    }
}

pub fn execute_command(
    game_state: &mut game_backend::GameState,
    command: &str,
) -> Result<String, String> {
    let argv = command
        .split_whitespace()
        .filter(|&s| !s.is_empty())
        .collect::<Vec<_>>();
    let command_name = argv.first().unwrap_or(&"");
    let command_box = get_command_by_name(command_name).unwrap_or(Box::new(InvalidCommand));
    if game_state.player_level < command_box.required_level() {
        Err(
            "You do not have access to run that command.\nThis incident will be reported."
                .to_string(),
        )
    } else {
        command_box.execute(game_state, &argv)
    }
}

pub fn list_commands(prefix: &str, level: i32) -> Vec<&str> {
    COMMAND_LIST
        .into_iter()
        .filter(|s| {
            s.starts_with(prefix) && get_command_by_name(s).unwrap().required_level() <= level
        })
        .collect()
}
