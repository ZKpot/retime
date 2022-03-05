use dotrix::ecs::{ Mut, Const, };
use dotrix::{ Window, Input, State as StateStack, };
use dotrix::overlay::Overlay;
use dotrix::window::Fullscreen;
use dotrix::math::{ Vec2u, };
use dotrix::egui::{
    self,
    Egui,
};

use crate::states;
use crate::actions;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum WindowMode {
    BorderlessFullscreen,
    Windowed,
}

pub struct State {
    pub show_info_panel: bool,
    window_mode: WindowMode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            show_info_panel: true,
            window_mode: WindowMode::Windowed,
        }
    }
}

pub fn startup(
    mut window: Mut<Window>,
) {
    window.set_inner_size(Vec2u::new(1280, 720));

    if let Err(e) = window.set_cursor_grab(true) {
        println!("Cannot grab cursor! {}", e);
    }
    window.set_cursor_visible(false);
}

pub fn menu(
    input: Const<Input>,
    overlay: Const<Overlay>,
    mut settings: Mut<State>,
    mut state_stack: Mut<StateStack>,
    mut window: Mut<Window>,
) {
    // toggle pause
    let mut paused = state_stack.get::<states::Pause>().is_some();

    if input.is_action_activated(actions::Action::Pause) {
        if paused {
            state_stack.pop_any();
        } else {
            state_stack.push(states::Pause {});
        }

        paused = !paused;

        if let Err(e) = window.set_cursor_grab(!paused) {
            println!("Cannot grab cursor! {}", e);
        }
        window.set_cursor_visible(paused);
    }

    // pause menu
    if paused {
        let egui = overlay.get::<Egui>()
            .expect("Renderer does not contain an Overlay instance");

        egui::containers::Window::new("Pause")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(130.0)
            .show(&egui.ctx, |ui| {
                ui.vertical_centered_justified(|ui| {

                    if ui.button("Resume").clicked() {
                        state_stack.pop_any();
                    }

                    if settings.show_info_panel == true {
                        if ui.button("Hide info panel").clicked() {
                            settings.show_info_panel = false;
                        }
                    } else {
                        if ui.button("Show info panel").clicked() {
                            settings.show_info_panel = true;
                        }
                    }

                    if settings.window_mode == WindowMode::BorderlessFullscreen {
                        if ui.button("Windowed").clicked() {
                            window.set_fullscreen(None);
                            settings.window_mode = WindowMode::Windowed;
                        }
                    } else {
                        if ui.button("Fullscreen").clicked() {
                            window.set_fullscreen(Some(Fullscreen::Borderless(0)));
                            settings.window_mode = WindowMode::BorderlessFullscreen;
                        }
                    }

                    if ui.button("Reset level").clicked() {
                        //state_stack.clear();
                        //state_stack.push(states::LevelInit {});
                    }

                    ui.add_space(25.0);

                    if ui.button("Exit").clicked() {
                        window.close();
                    }
                })
            });
    }
}
