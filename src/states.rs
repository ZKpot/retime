use dotrix::{
    State,
    ecs::{ Mut, Const, },
};

use crate::physics;

// Services
pub struct Stats {
    pub time: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            time: 0.0,
        }
    }
}

// States
pub struct Pause {}

pub struct LevelInit {}

pub struct RunLevel {}

pub struct RewindTime {}

// Systems
pub fn after_init(
    mut state: Mut<State>,
) {
    state.clear();
    state.push(RunLevel {});
}

pub fn update (
    mut stats: Mut<Stats>,
    physics_state: Const<physics::State>,
) {
    stats.time += physics_state.physics.as_ref()
        .expect("physics_state must be defined").integration_parameters.dt;
}
