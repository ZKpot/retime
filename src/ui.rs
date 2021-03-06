use dotrix::ecs::{ Mut, Const, Context, };
use dotrix::{ Window, Input, State as StateStack, Frame, Assets, World, };
use dotrix::overlay::Overlay;
use dotrix::window::Fullscreen;
use dotrix::math::{ Vec2u, Vec3, };
use dotrix::egui::{
    self,
    Egui,
};

use crate::states;
use crate::actions;
use crate::time;
use crate::level;
use crate::ui_clock::Clock;
use std::f32::consts::PI;
use std::fs;

pub struct Ctx {
    level_list: Vec<String>,
    frame: egui::containers::Frame,
    offset: f32,
}

impl Default for Ctx {
    fn default() -> Self {
        let frame = egui::containers::Frame {
            fill: egui::Color32::from_gray(0),
            stroke: egui::Stroke::none(),
            ..Default::default()
        };

        let level_folder = "levels/";

        let mut level_list = Vec::new();

        for file in fs::read_dir(["./", level_folder].join("")).unwrap() {
            level_list.push(file.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string());
        }

        Self {
            level_list,
            frame,
            offset: 10.0,
        }
    }
}

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
    mut window: Mut<Window>,
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

pub fn draw_main_menu(
    ctx: Context<Ctx>,
    overlay: Const<Overlay>,
    mut state_stack: Mut<StateStack>,
    mut stats_opt: Mut<Option<states::Stats>>,
    mut level_opt: Mut<Option<level::Level>>,
    mut window: Mut<Window>,
) {
    let state = state_stack.get_mut::<states::MainMenu>()
        .expect("something terrible has happened");

    if !state.selected_level.is_some() {
        let egui = overlay.get::<Egui>()
            .expect("Renderer does not contain an Overlay instance");

        egui::containers::Window::new("Main menu")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
            .frame(ctx.frame)
            .collapsible(false)
            .resizable(false)
            .default_width(160.0)
            .show(&egui.ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    for level in &ctx.level_list {
                        if ui.button(level).clicked() {
                            state.selected_level = Some(level.to_string());
                        }
                    }

                    ui.add_space(25.0);

                    if ui.button("Exit").clicked() {
                        window.close();
                    }
                });
            });
    }

    if let Some(level) = &state.selected_level {
        let level = level::Level::from_file(&[level, ".yaml"].join(""));
        *stats_opt = Some(states::Stats::new(
            Vec3::new(level.target_position.0, level.target_position.1, level.target_position.2)
        ));
        *level_opt = Some(level);
        state_stack.push(states::LoadAssets::default());
    }
}

pub fn draw_loading_screen(
    ctx: Context<Ctx>,
    overlay: Const<Overlay>,
) {
    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    egui::containers::Window::new("Loading screen")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::new(ctx.offset, -ctx.offset))
        .frame(ctx.frame)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .show(&egui.ctx, |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new("ReTime: Loading...").heading()
            ));
        });
}

pub fn draw_menu(
    input: Const<Input>,
    overlay: Const<Overlay>,
    mut settings: Mut<State>,
    mut state_stack: Mut<StateStack>,
    mut window: Mut<Window>,
    frame: Const<Frame>,
    stats_opt: Const<Option<states::Stats>>,
) {
    let stats = stats_opt.as_ref()
        .expect("Game stats should be initialized");

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
                        while !state_stack.get::<states::MainMenu>().is_some() {
                            state_stack.pop_any();
                        }
                    }

                    if ui.button("Main menu").clicked() {
                        state_stack.clear();
                        state_stack.push(states::MainMenu {selected_level: None});
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

pub fn draw_in_game_panels(
    ctx: Context<Ctx>,
    world: Const<World>,
    overlay: Const<Overlay>,
    stats_opt: Const<Option<states::Stats>>,
    time_stack: Const<time::Stack>,
    mut assets: Mut<Assets>,
) {
    let stats = stats_opt.as_ref()
        .expect("Game stats should be initialized");

    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    let game_frame = egui::containers::Frame{
        fill: egui::Color32::from_black_alpha(192),
        corner_radius: 2.5,
        margin: egui::Vec2::new(4.0, 4.0),
        ..Default::default()
    };

    let scale = time::STACK_MAX_SIZE as f32;

    egui::containers::Window::new("clock")
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-ctx.offset, ctx.offset))
        .frame(game_frame)
        .resizable(false)
        .title_bar(false)
        .show(&egui.ctx, |ui| {
            ui.add(
                Clock::new(
                    (time_stack.count-time_stack.index_cleared) as f32 / scale*2.0*PI,
                    (time_stack.index-time_stack.index_cleared) as f32 / scale*2.0*PI,
                    (time_stack.index_max+time_stack.index-time_stack.di-time_stack.index_cleared) as f32 /
                        scale*2.0*PI,
                )
            );
        });

        egui::containers::Window::new("score")
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(ctx.offset, ctx.offset))
            .frame(game_frame)
            .resizable(false)
            .title_bar(false)
            .show(&egui.ctx, |ui| {
                ui.add(egui::Label::new(egui::RichText::new(format!("{:04.1}", stats.time))
                    .color(egui::Color32::GRAY)
                    .heading()
                ));
            });

    // actionable objects panel
    let image_size = 60.0;

    egui::containers::Window::new("items_menu")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-ctx.offset, -ctx.offset))
        .frame(game_frame)
        .resizable(false)
        .title_bar(false)
        .show(&egui.ctx, |ui| {
            egui::Grid::new("grid")
                .show(ui, |ui| {

                    // query player
                    let query = world.query::<(&mut time::ActionableObject,)>();

                    for (object,) in query {

                        let bg_color = if object.selected {
                            egui::Color32::DARK_GRAY
                        } else {
                            egui::Color32::TRANSPARENT
                        };

                        let alpha = if object.active {
                            255
                        } else {
                            32
                        };

                        ui.add(
                            egui::Image::new(
                                egui::TextureId::User(
                                    assets.register::<dotrix::assets::Texture>
                                        (object.tile_texture_name)
                                        .as_u64().expect("Expected id as u64")
                                ),
                                egui::Vec2::new(image_size, image_size)
                            ).tint(egui::Color32::from_white_alpha(alpha))
                                .bg_fill(bg_color)
                        );
                    }
                });
        });
}

pub fn draw_background(
    overlay: Const<Overlay>,
) {
    //loading screen
    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    let frame = egui::containers::Frame{
        fill: egui::Color32::from_gray(0),
        ..Default::default()
    };

    egui::containers::panel::CentralPanel::default()
        .frame(frame)
        .show(&egui.ctx, |_| {});
}
