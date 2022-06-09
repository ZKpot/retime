use dotrix::{
    State, World, Transform,
    ecs::{ Mut, Const, },
    math::{ Vec3, },
};

use crate::physics;
use crate::player;

// Services
pub struct Stats {
    pub time: f32,
    pub level_passed: bool,
    pub finish_pos: Vec3,
}

impl Stats {
    pub fn new(finish_pos: Vec3) -> Self {
        Self {
            time: 0.0,
            level_passed: false,
            finish_pos,
        }
    }
}

// States
pub struct MainMenu {
    pub selected_level: Option<String>,
}

pub struct Pause {
    pub handled: bool,
}

impl Default for Pause {
    fn default() -> Self {
        Self {
            handled: false,
        }
    }
}

pub struct LoadAssets {
    pub imported: bool,
    pub time_left_secs: f32,
}

impl Default for LoadAssets {
    fn default() -> Self {
        Self {
            imported: false,
            time_left_secs: 0.3,
        }
    }
}

pub struct InitLevel {}

pub struct RunLevel {}

pub struct RewindTime {}

// Systems
pub fn after_init(
    mut state: Mut<State>,
) {
    state.push(RunLevel {});
}

pub fn update (
    mut stats_opt: Mut<Option<Stats>>,
    physics_state: Const<physics::State>,
    world: Const<World>,
    mut state: Mut<State>,
) {
    let mut stats = stats_opt.as_mut()
        .expect("Game stats should be initialized");

    stats.time += physics_state.physics.as_ref()
        .expect("physics_state must be defined").integration_parameters.dt;

    let query = world.query::<(
        &Transform, &player::State,
    )>();

    for (transform, _) in query {
        if (
            (transform.translate.x - stats.finish_pos.x).powi(2) +
            (transform.translate.y - stats.finish_pos.y).powi(2) +
            (transform.translate.z - stats.finish_pos.z).powi(2)
        ).sqrt() <= 2.0 {
            stats.level_passed = true;
            state.push(Pause::default());
        }
    }
}
