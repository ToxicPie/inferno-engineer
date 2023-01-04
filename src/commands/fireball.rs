use crate::{commands, game_backend, npcs};

pub struct FireballCommand;

impl commands::GameCommand for FireballCommand {
    fn synopsis(&self) -> &'static str {
        "fireball [damage]"
    }
    fn man_page(&self) -> &'static str {
        r#"fireball - Summon a fireball

SYNOPSIS
    fireball [damage]

DESCRIPTION
    Throw a fireball at your enemy that deals the damage amount specified.
    The number must be an positive integer not larger than your ATK stat.
    If `damage' is omitted, deal damage equal to your ATK.

EXAMPLES
    fireball 10
        Summon a fireball that deals 10 damage to the enemy.
"#
    }
    fn required_level(&self) -> i32 {
        2
    }
    fn execute(
        &self,
        game_state: &mut game_backend::GameState,
        argv: &[&str],
    ) -> Result<String, String> {
        let damage = if let Some(damage) = argv.get(1) {
            match damage.parse::<i32>() {
                Ok(damage) if 0 < damage => damage,
                Ok(_) => {
                    return Err(format!(
                        "Damage should be positive and not larger than your ATK"
                    ))
                }
                Err(_) => return Err(format!("`{}' is not a valid number", damage)),
            }
        } else {
            game_state.player_atk
        };
        if game_state.in_battle {
            game_state
                .action_queue
                .push(npcs::PlayerAction::Attack(damage));
            Ok(format!("Dealt {} damage", damage))
        } else {
            Err("Not in battle".to_string())
        }
    }
}
