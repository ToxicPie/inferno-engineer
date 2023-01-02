use crate::commands;
use crate::gameplay;

pub struct ManCommand {}

impl commands::GameCommand for ManCommand {
    fn synopsis(&self) -> &'static str {
        "man <command_name>"
    }
    fn man_page(&self) -> &'static str {
        r#"man - Display a command's manual

SYNOPSIS
    man <command_name>
    manual <command_name>

DESCRIPTION
    Show detailed help about the command given.
    For a brief description, use the "help" command.

EXAMPLES
    man attack
        Show the manual of the command "attack".
"#
    }
    fn required_level(&self) -> i32 {
        i32::MIN
    }
    fn execute(
        &self,
        game_state: &mut gameplay::GameState,
        argv: &[&str],
    ) -> Result<String, String> {
        if let Some(command_name) = argv.get(1) {
            if let Some(command_box) = commands::get_command_by_name(command_name) {
                if game_state.player_level < command_box.required_level() {
                    Err("You don't have access to that command".to_string())
                } else {
                    Ok(command_box.man_page().to_string())
                }
            } else {
                Err(format!("No such command: {}", command_name))
            }
        } else {
            Err(format!("Usage: {}", self.synopsis()))
        }
    }
}
