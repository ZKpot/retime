use dotrix::ecs::{ Mut, Const, };
use dotrix::{ Window, Input, State as StateStack, Frame, Assets, };
use dotrix::overlay::Overlay;
use dotrix::window::Fullscreen;
use dotrix::math::{ Vec2u, };
use dotrix::egui::{
    self,
    Egui,
};

use crate::states;
use crate::actions;
use crate::time;

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
            show_info_panel: false,
            window_mode: WindowMode::Windowed,
        }
    }
}

pub fn startup(
    window: Const<Window>,
) {
    window.set_inner_size(Vec2u::new(1280, 720));
}

pub fn init(
    mut window: Mut<Window>,
) {
    if let Err(e) = window.set_cursor_grab(true) {
        println!("Cannot grab cursor! {}", e);
    }
    window.set_cursor_visible(false);
}

pub fn draw_menu(
    input: Const<Input>,
    overlay: Const<Overlay>,
    mut settings: Mut<State>,
    mut state_stack: Mut<StateStack>,
    mut window: Mut<Window>,
    frame: Const<Frame>,
    stats: Const<states::Stats>,
) {
    // toggle pause
    let mut paused = state_stack.get::<states::Pause>().is_some();
    let mut state_changed = false;

    if input.is_action_activated(actions::Action::Pause) && !stats.level_passed {
        if paused {
            state_stack.pop_any();
        } else {
            state_stack.push(states::Pause::default());
        }

        paused = !paused;
        state_changed = true;
    }

    if let Some(pause_state) = state_stack.get_mut::<states::Pause>() {
        if !pause_state.handled{
            pause_state.handled = true;
            state_changed = true;
        }
    }

    if state_changed {
        if let Err(e) = window.set_cursor_grab(!paused) {
            println!("Cannot grab cursor! {}", e);
        }
        window.set_cursor_visible(paused);
    }


    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    // pause menu
    let label = if stats.level_passed {
        format!("Level passed in {:04.1} secs", stats.time)
    } else {
        "Pause".to_string()
    };

    if paused {
        egui::containers::Window::new(label)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(130.0)
            .show(&egui.ctx, |ui| {
                ui.vertical_centered_justified(|ui| {

                    if !stats.level_passed {
                        if ui.button("Resume").clicked() {
                            state_stack.pop_any();
                        }
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
                        state_stack.push(states::LevelInit {});
                    }

                    ui.add_space(25.0);

                    if ui.button("Exit").clicked() {
                        window.close();
                    }
                })
            });
    }

    // info panel
    if settings.show_info_panel {

        let info_frame = egui::containers::Frame{
            fill: egui::Color32::from_black_alpha(192),
            corner_radius: 2.5,
            margin: egui::Vec2::new(4.0, 4.0),
            ..Default::default()
        };

        let egui = overlay.get::<Egui>()
            .expect("Renderer does not contain an Overlay instance");

        if settings.show_info_panel {
            egui::SidePanel::left("info_panel")
                .resizable(false)
                .frame(info_frame)
                .show(&egui.ctx, |ui| {
                    egui::Grid::new("info_grid").show(ui, |ui| {
                        let color = if paused {
                            egui::Color32::LIGHT_GRAY
                        } else {
                            egui::Color32::GRAY
                        };
                        ui.add(egui::Label::new(
                            egui::RichText::new("FPS").color(color)
                        ));
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("{:05.1}", frame.fps()))
                                .color(color)
                        ));
                        ui.end_row();

                        let states_stack_dump = state_stack.dump().join(",\n  ");
                        ui.add(egui::Label::new(
                            egui::RichText::new(
                                format!("Current state stack: \n  {}\n", states_stack_dump)
                            ).color(color)
                        ));
                    });
                });
        }
    }
}

pub fn draw_panel(
    overlay: Const<Overlay>,
    stats: Const<states::Stats>,
    time_stack: Const<time::Stack>,
    mut assets: Mut<Assets>,
) {
    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    // time bar and score
    let offset = 10.0;

    let game_frame = egui::containers::Frame{
        fill: egui::Color32::from_black_alpha(192),
        corner_radius: 2.5,
        margin: egui::Vec2::new(4.0, 4.0),
        ..Default::default()
    };

    let time_bar = (time::STACK_MAX_SIZE - time_stack.index) as f32 /
        time::STACK_MAX_SIZE as f32;

    let time_bar_width = 20.0 + 140.0 * time_stack.physics_state.len() as f32 /
        time::STACK_MAX_SIZE as f32;

    egui::containers::Window::new("game_bar")
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0.0, offset))
        .frame(game_frame)
        .resizable(false)
        .title_bar(false)
        .show(&egui.ctx, |ui| {
            egui::Grid::new("score")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_width(170.0);
                        ui.add(
                            egui::widgets::ProgressBar::new(time_bar)
                                .desired_width(time_bar_width)
                        );
                    });

                    ui.add(egui::Label::new(egui::RichText::new(format!("{:04.1}", stats.time))
                        .color(egui::Color32::GRAY)
                        .heading()
                    ));
                });
        });

    // actionable objects panel
    let image_size = 60.0;

    egui::containers::Window::new("items_menu")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-offset, -offset))
        .frame(game_frame)
        .resizable(false)
        .title_bar(false)
        .show(&egui.ctx, |ui| {
            egui::Grid::new("grid")
                .show(ui, |ui| {

                    ui.add(
                        egui::Image::new(
                            egui::TextureId::User(
                                assets.register::<dotrix::assets::Texture>("player")
                                    .as_u64().expect("Expected id as u64")
                            ),
                            egui::Vec2::new(image_size, image_size)
                        ).tint(egui::Color32::from_white_alpha(255))
                            .bg_fill(egui::Color32::DARK_GRAY)
                    );

                    ui.add(
                        egui::Image::new(
                            egui::TextureId::User(
                                assets.register::<dotrix::assets::Texture>("trampoline")
                                    .as_u64().expect("Expected id as u64")
                            ),
                            egui::Vec2::new(image_size, image_size)
                        ).tint(egui::Color32::from_white_alpha(32))
                            .bg_fill(egui::Color32::TRANSPARENT)
                    );
                });
        });
}
