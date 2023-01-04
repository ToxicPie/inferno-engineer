use crate::{commands, game_backend, npcs};

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_startup_system(prepare_ui)
            .add_system(game_ui)
            .add_system(update_ui_events);
    }
}

fn prepare_ui(mut egui_context: ResMut<EguiContext>, mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        egui_context
            .ctx_for_window_mut(window.id())
            .set_visuals(egui::Visuals::dark());
        window.set_title("Inferno Engineer".to_string());
        window.set_present_mode(bevy::window::PresentMode::AutoVsync);
        window.set_scale_factor_override(Some(1.0));
    }
}

fn game_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    game_state: ResMut<game_backend::GameState>,
    mut active_npc: ResMut<game_backend::ActiveNpc>,
    command_events: EventWriter<game_backend::CommandExecutionEvent>,
    npc_events: EventWriter<game_backend::NpcActionEvent>,
) {
    let mut is_terminal_open = ui_state.is_terminal_open;

    let (show_info, show_terminal) = {
        use game_backend::GameProgress::*;
        match game_state.game_progress {
            HasTerminal => (true, true),
            HasPanel => (true, false),
            Tutorial | Intro => (false, false),
        }
    };

    if show_info {
        // the info window
        egui::Window::new("Info")
            .collapsible(true)
            .show(egui_context.ctx_mut(), |ui| {
                // show a tab bar that allows selecting a panel
                egui::TopBottomPanel::bottom("tab_bar")
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Dialogue").clicked() {
                                ui_state.selected_tab = InfoTab::Dialogue;
                            }
                            if ui.button("Details").clicked() {
                                ui_state.selected_tab = InfoTab::Details;
                            }
                            if show_terminal {
                                ui.checkbox(&mut is_terminal_open, "Show terminal");
                            }
                        })
                    });

                egui::CentralPanel::default().show_inside(ui, |ui| match ui_state.selected_tab {
                    InfoTab::Dialogue => game_ui_dialogue(ui, ui_state.as_mut(), npc_events),
                    InfoTab::Details => game_ui_details(ui, game_state.as_ref(), &mut active_npc),
                });
            });
    }

    if show_terminal {
        // the terminal window
        egui::Window::new("Terminal")
            .collapsible(true)
            .open(&mut is_terminal_open)
            .show(egui_context.ctx_mut(), |ui| {
                game_ui_terminal(ui, ui_state.as_mut(), game_state.as_ref(), command_events)
            });
        ui_state.is_terminal_open = is_terminal_open;
    }
}

fn game_ui_dialogue(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    mut npc_events: EventWriter<game_backend::NpcActionEvent>,
) {
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for (name, text) in ui_state.dialogue.iter() {
                ui.horizontal_wrapped(|ui| {
                    if let Some(name) = name {
                        let color = match name.as_str() {
                            "You" => egui::Color32::from_rgb(80, 140, 242),
                            _ => egui::Color32::from_rgb(226, 45, 42),
                        };
                        ui.label(
                            egui::RichText::new(name)
                                .color(color)
                                .strong()
                                .underline(),
                        );
                    }
                    ui.label(text);
                });
            }

            if ui_state.choices.is_empty() {
                if ui.button("Next").clicked() {
                    npc_events.send_default();
                }
            } else {
                let mut chosen = false;
                for (idx, choice) in ui_state.choices.iter().enumerate() {
                    if ui.button(choice).clicked() {
                        chosen = true;
                        ui_state.dialogue.push((Some("You".to_string()), choice.to_owned()));
                        npc_events.send(game_backend::NpcActionEvent(npcs::PlayerAction::Respond(
                            idx,
                        )));
                    }
                }
                if chosen {
                    ui_state.choices.clear();
                }
            }
        });
}

fn game_ui_details(
    ui: &mut egui::Ui,
    game_state: &game_backend::GameState,
    active_npc: &game_backend::ActiveNpc,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label(match &active_npc.0 {
            Some(npc) => npc.info(),
            None => "You are exploring by yourself".to_string(),
        });
        ui.separator();
        ui.label(game_state.player_details());
    });
}

fn game_ui_terminal(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    game_state: &game_backend::GameState,
    mut command_events: EventWriter<game_backend::CommandExecutionEvent>,
) {
    // the input panel at the button
    egui::TopBottomPanel::bottom("input_panel")
        .resizable(false)
        .min_height(0.0)
        .show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                // show a textbox for typing commands and a "run" button
                let command_button = ui.button("Run");
                let command_input = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut ui_state.terminal_input)
                        .code_editor()
                        .hint_text("Type a command..."),
                );
                ui_state.is_textbox_focused = command_input.has_focus();

                // run the command when enter key or button is pressed
                if command_button.clicked()
                    || (command_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                {
                    let command = ui_state.terminal_input.trim().to_string();
                    if !command.is_empty() {
                        ui_state.log_message(format!(">>> {}", command));
                        ui_state.terminal_input.clear();
                        command_input.request_focus();
                        command_events.send(game_backend::CommandExecutionEvent(command));
                    }
                }

                // show command completion or hints
                let hint_popup_id = ui.make_persistent_id("command_hint");
                egui::popup::popup_above_or_below_widget(
                    ui,
                    hint_popup_id,
                    &command_input,
                    egui::AboveOrBelow::Below,
                    |ui| {
                        let user_input = ui_state.terminal_input.clone();
                        let command = user_input
                            .split_whitespace()
                            .find(|s| !s.is_empty())
                            .unwrap_or_default()
                            .to_lowercase();
                        let command = command.as_str();

                        // function that completes user input
                        let mut do_completion = |ctx, completion: &str| {
                            // set content, get focus and move the cursor to the end
                            ui_state.terminal_input = completion.to_string();
                            command_input.request_focus();
                            if let Some(mut state) =
                                egui::TextEdit::load_state(ctx, command_input.id)
                            {
                                let ccursor = egui::text::CCursor::new(completion.len());
                                state.set_ccursor_range(Some(egui::text::CCursorRange::one(
                                    ccursor,
                                )));
                                state.store(ctx, command_input.id);
                            }
                        };

                        match commands::get_command_by_name(command) {
                            Some(command_box) => {
                                // a command name is typed, show help of the command
                                ui.monospace(command_box.synopsis());
                            }
                            None => {
                                // otherwise show command completions (max 5)
                                let mut completions =
                                    commands::list_commands(command, game_state.player_level);
                                completions.sort();
                                let mut completion: Option<&str> = None;

                                if command_input.has_focus()
                                    && ui.input().key_pressed(egui::Key::Tab)
                                {
                                    // the user can press tab to complete a command
                                    if let Some(candidate) = completions.first() {
                                        completion = Some(candidate);
                                    }
                                } else {
                                    // or click the command's button
                                    for candidate in completions.iter().take(5) {
                                        if ui
                                            .button(egui::RichText::new(*candidate).monospace())
                                            .clicked()
                                        {
                                            completion = Some(candidate);
                                        }
                                    }
                                }
                                if let Some(completion_str) = completion {
                                    do_completion(ui.ctx(), completion_str);
                                }
                            }
                        };
                    },
                );
                // show the completion popup if the textbox has focus
                if command_input.has_focus() {
                    ui.memory().open_popup(hint_popup_id);
                } else {
                    ui.memory().close_popup();
                }
            });
        });

    // show command log in the main panel
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                let log_string = ui_state.get_log_string();
                let mut log_str = log_string.as_str();
                let log_text = egui::TextEdit::multiline(&mut log_str)
                    .code_editor()
                    .desired_width(f32::INFINITY);
                ui.add_sized(ui.available_size(), log_text);
            });
    });
}

fn update_ui_events(
    mut ui_state: ResMut<UiState>,
    mut command_events: EventReader<game_backend::CommandResultEvent>,
    mut npc_events: EventReader<game_backend::NpcResponseEvent>,
) {
    for game_backend::CommandResultEvent(result) in command_events.iter() {
        ui_state.log_message(result.to_owned() + "\n");
    }
    for game_backend::NpcResponseEvent(response) in npc_events.iter() {
        let npcs::NpcResponse {
            message,
            name,
            choices,
        } = response;
        ui_state
            .dialogue
            .push((name.to_owned(), message.to_owned()));
        ui_state.choices = choices.to_owned();
    }
}

enum InfoTab {
    Dialogue,
    Details,
}

#[derive(Resource)]
pub struct UiState {
    terminal_input: String,
    terminal_log: Vec<String>,
    dialogue: Vec<(Option<String>, String)>,
    choices: Vec<String>,
    selected_tab: InfoTab,
    is_terminal_open: bool,
    pub is_textbox_focused: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            terminal_input: String::new(),
            terminal_log: vec![
                "The Inferno Interactive Console v666".to_string(),
                "Type `commands` for a list of commands or try `help <command>`.".to_string(),
            ],
            dialogue: vec![],
            choices: vec![],
            selected_tab: InfoTab::Dialogue,
            is_terminal_open: false,
            is_textbox_focused: false,
        }
    }
}

impl UiState {
    const MAX_LOG_LINES: usize = 256;
    fn log_message(&mut self, message: String) {
        self.terminal_log.push(message);
        // limit output to MAX_LOG_LINES messages
        if self.terminal_log.len() > UiState::MAX_LOG_LINES {
            self.terminal_log =
                self.terminal_log[self.terminal_log.len() - UiState::MAX_LOG_LINES..].to_vec();
        }
    }
    fn get_log_string(&self) -> String {
        self.terminal_log.join("\n")
    }
}
