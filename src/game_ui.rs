use crate::commands;
use crate::gameplay;

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>().add_system(game_ui);
    }
}

fn game_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut game_state: ResMut<gameplay::GameState>,
) {
    // the terminal window
    egui::Window::new("Terminal").show(egui_context.ctx_mut(), |ui| {
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
                        egui::TextEdit::singleline(&mut ui_state.input)
                            .code_editor()
                            .hint_text("Type a command..."),
                    );

                    // run the command when enter key or button is pressed
                    if command_button.clicked()
                        || (command_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                    {
                        let command = ui_state.input.trim().clone();
                        if !command.is_empty() {
                            let message = match commands::execute_command(&mut game_state, command)
                            {
                                Ok(msg) => msg,
                                Err(msg) => format!("Error: {}", msg),
                            };
                            let execution = format!(">>> {}", command);
                            ui_state.log.push(execution);
                            ui_state.log.push(message + "\n");
                            // limit output to MAX_LOG_LINES messages
                            if ui_state.log.len() > UiState::MAX_LOG_LINES {
                                ui_state.log = ui_state.log
                                    [ui_state.log.len() - UiState::MAX_LOG_LINES..]
                                    .to_vec();
                            }
                            ui_state.input.clear();
                            command_input.request_focus();
                        }
                    }

                    // show command completion or hints
                    let hint_popup_id = ui.make_persistent_id("command_hint");
                    egui::popup::popup_above_or_below_widget(
                        ui,
                        hint_popup_id,
                        &command_input,
                        egui::AboveOrBelow::Above,
                        |ui| {
                            let user_input = ui_state.input.clone();
                            let command = user_input
                                .split_whitespace()
                                .find(|s| !s.is_empty())
                                .unwrap_or_default();

                            // function that completes user input
                            let mut do_completion = |ctx, completion: &str| {
                                // set content, get focus and move the cursor to the end
                                ui_state.input = completion.to_string();
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
                    let log_string = ui_state.log.join("\n");
                    let mut log_str = log_string.as_str();
                    let log_text = egui::TextEdit::multiline(&mut log_str)
                        .code_editor()
                        .desired_width(f32::INFINITY);
                    ui.add_sized(ui.available_size(), log_text);
                });
        });
    });
}

#[derive(Resource)]
pub struct UiState {
    input: String,
    log: Vec<String>,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            input: String::new(),
            log: vec![
                "The Inferno Terminal v666".to_string(),
                "Type `commands` for a list of commands or try `help <command>`.".to_string(),
            ],
        }
    }
}

impl UiState {
    const MAX_LOG_LINES: usize = 256;
}
