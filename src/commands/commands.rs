use crate::commands;
use crate::gameplay;

pub struct CommandsCommand {}

impl commands::Command for CommandsCommand {
    fn synopsis(&self) -> &'static str {
        "commands [-v]"
    }
    fn man_page(&self) -> &'static str {
        r#"commands - Show available commands

SYNOPSIS
    commands [-v]

DESCRIPTION
    Show a list of commands.

    -v
        Also show the synopses of commands.
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
        let commands = commands::list_commands("", game_state.player_level);
        if argv.contains(&"-v") {
            Ok(commands
                .iter()
                .map(|name| commands::get_command_by_name(name).unwrap().synopsis())
                .collect::<Vec<_>>()
                .join("\n"))
        } else {
            Ok(commands.join(" "))
        }
    }
}
