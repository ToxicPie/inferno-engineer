use crate::commands;
use crate::gameplay;

pub struct HelpCommand {}

impl commands::GameCommand for HelpCommand {
    fn synopsis(&self) -> &'static str {
        "help <command_name>"
    }
    fn man_page(&self) -> &'static str {
        r#"help - Display help about a command

SYNOPSIS
    help <command_name>

DESCRIPTION
    Display a brief help about the command given.
    For a more detailed description, use the "man" command.

EXAMPLES
    help attack
        Show help about the command "attack".
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
                    Ok(command_box.synopsis().to_string())
                }
            } else {
                Err(format!("No such command: {}", command_name))
            }
        } else {
            Err(format!("Usage: {}", self.synopsis()))
        }
    }
}
