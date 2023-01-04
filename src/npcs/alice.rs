use crate::npc_response;
use crate::{game_backend, npcs};
use npcs::Npc;
use std::collections::VecDeque;

pub struct AliceNpc {
    progress: usize,
    previous_choice: usize,
    hitpoints: i32,
    message_queue: VecDeque<npcs::NpcResponse>,
}

impl Default for AliceNpc {
    fn default() -> Self {
        AliceNpc {
            progress: 0,
            previous_choice: 0,
            hitpoints: i32::MAX,
            message_queue: VecDeque::new(),
        }
    }
}

impl npcs::Npc for AliceNpc {
    fn id(&self) -> &'static str {
        "alice"
    }
    fn name(&self) -> &'static str {
        if self.progress >= 17 {
            "BreeDFS"
        } else {
            "???"
        }
    }
    fn info(&self) -> String {
        if self.progress >= 17 {
            [
                "BreeDFS",
                "A floating sphere resembling the BreeDFS logo.",
                "The ultimate form of evil, overlord of hell.",
            ]
            .join("\n")
        } else {
            "???".to_string()
        }
    }
    fn handle_action(
        &mut self,
        action: &npcs::PlayerAction,
        game_state: &mut game_backend::GameState,
    ) -> () {
        match action {
            npcs::PlayerAction::Ping => self.interact(game_state, None),
            npcs::PlayerAction::Attack(damage) => {
                self.hitpoints -= damage;
            }
            npcs::PlayerAction::Respond(choice) => self.interact(game_state, Some(*choice)),
        }
    }
    fn get_response(&mut self) -> Option<npcs::NpcResponse> {
        if let Some(res) = self.message_queue.pop_front() {
            Some(res)
        } else {
            None
        }
    }
    fn job_completed(&self) -> bool {
        self.progress >= 25
    }
}

impl AliceNpc {
    fn interact(&mut self, game_state: &mut game_backend::GameState, choice: Option<usize>) {
        if let Some(num) = choice {
            self.previous_choice = num;
        }
        let choice = self.previous_choice;
        match self.progress {
            0 => {
                self.message_queue.push_back(npc_response!(
                    "oh hi human being! welcome to hell!",
                    self.name()
                ));
                game_state.game_progress = game_backend::GameProgress::HasPanel;
            }
            1 => {
                self.message_queue.push_back(npc_response!(
                    "you're new here, right?";
                    "Yeah", "I guess...?",
                ));
            }
            2 => {
                self.message_queue.push_back(npc_response!(
                    "oh cool! do you remember what happened?",
                    self.name();
                    "No"
                ));
            }
            3 => {
                self.message_queue.push_back(npc_response!(
                    "hmm. i suppose there's this possibility -",
                    self.name()
                ));
            }
            4 => {
                self.message_queue.push_back(npc_response!(
                    "that you have just died.";
                    "What?", "Wait I remember! There was a truck..."
                ));
            }
            5 => {
                if choice == 0 {
                    self.message_queue.push_back(npc_response!(
                        "this is the inferno. a place where decease souls and other creatures belong.",
                        self.name()
                    ));
                } else {
                    self.message_queue.push_back(npc_response!(
                        "so it seems like you do remember...",
                        self.name()
                    ));
                    self.progress += 1;
                }
            }
            6 => {
                if choice == 0 {
                    self.message_queue.push_back(npc_response!(
                        "i'm sorry, human. but i have some bad news. you have just died."
                    ));
                }
            }
            7 => {
                self.message_queue.push_back(npc_response!("...."));
            }
            8 => {
                self.message_queue.push_back(npc_response!(
                    "anyways, the hell is currently undergoing some system upgrades."
                ));
            }
            9 => {
                self.message_queue.push_back(npc_response!(
                    "things have been going really, *really* bad lately."
                ));
                self.message_queue.push_back(npc_response!(
                    "bugs are everywhere, and even the most overworked workers couldn't fix them."
                ));
                self.message_queue.push_back(npc_response!(
                    "even worse, at least half of them quit their jobs last month."
                ));
            }
            10 => {
                self.message_queue.push_back(npc_response!(
                    "i've heard about you before. you were an engineer, right?";
                    "Yes.", "No?"
                ));
            }
            11 => {
                if choice == 0 {
                    self.message_queue
                        .push_back(npc_response!("cool!!!", self.name()));
                } else {
                    self.message_queue
                        .push_back(npc_response!("liars will be burning in hell!", self.name()));
                }
            }
            12 => {
                self.message_queue.push_back(npc_response!(
                    "so as i said, we kind of need a new maintainer of our technology systems, stat."
                ));
                self.message_queue.push_back(npc_response!(
                    "are you interested in helping us?";
                    "yes", "Yes", "YES", "YES", "YES"
                ));
            }
            13 => {
                self.message_queue.push_back(npc_response!(
                    "OMG THANKS!!1!1! i knew you would help me, kind human!!",
                    self.name();
                    "??????"
                ));
            }
            14 => {
                self.message_queue.push_back(npc_response!(
                    "from now on, you are our new system administrator!",
                    self.name()
                ));
                self.message_queue.push_back(npc_response!(
                    "do you think you are qualified for this job?";
                    "yeah!", "Of course!", "Definitely!"
                ));
            }
            15 => {
                self.message_queue.push_back(npc_response!(
                    "ok! i'll introduce your job to you soon.",
                    self.name();
                    "Wait you're cheating!", "I didn't have a choice..."
                ));
            }
            16 => {
                self.message_queue.push_back(npc_response!(
                    "oh of course you don't have a choice.",
                    self.name()
                ));
                self.message_queue
                    .push_back(npc_response!("i am a literal god. i control this place."));
            }
            17 => {
                self.message_queue.push_back(npc_response!(
                    "anyways, my name is BreeDFS. nice to meet you!",
                    self.name();
                    "Oh, that's why you looked very familiar..."
                ));
            }
            18 => {
                self.message_queue
                    .push_back(npc_response!("...what??", self.name()));
            }
            19 => {
                self.message_queue.push_back(npc_response!("...."));
                self.message_queue
                    .push_back(npc_response!("let's just get to the point."));
            }
            20 => {
                self.message_queue.push_back(npc_response!(
                    "to help you do your job, i have unlocked a new feature for you."
                ));
                self.message_queue.push_back(npc_response!(
                    "see the \"show terminal\" checkbox? click on it and see what happens."
                ));
                game_state.game_progress = game_backend::GameProgress::HasTerminal;
            }
            21 => {
                self.message_queue
                    .push_back(npc_response!("isn't it cool?"));
                self.message_queue.push_back(npc_response!(
                    "the Terminal is what we use to do our jobs efficiently."
                ));
                self.message_queue.push_back(npc_response!(
                    "we usually use \"commands\" to complete our tasks."
                ));
                self.message_queue.push_back(npc_response!(
                    "for example, right now you can try some simple commands like `help`.";
                    "Nice."
                ));
            }
            22 => {
                self.message_queue.push_back(npc_response!(
                    "as your \"access level\" increases, you will unlock more powerful commands.",
                    self.name()
                ));
                self.message_queue.push_back(npc_response!(
                    "you can click on the \"details\" button to see your access level as well as some other stats."
                ));
            }
            23 => {
                self.message_queue.push_back(npc_response!(
                    "although you only have a few commands available now, you should really take your time to familiarize yourself with the terminal!"
                ));
                self.message_queue.push_back(npc_response!(
                    "after you've mess around enough, press the \"OK\" button below.";
                    "OK"
                ));
            }
            24 => {
                self.message_queue.push_back(npc_response!(
                    "that's about it! i gotta leave now though... the rest is up to you!"
                ));
            }
            _ => unreachable!(),
        }
        self.progress += 1;
    }
}
