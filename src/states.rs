use dotrix::{
    State, World, Transform,
    ecs::{ Mut, Const, },
};

use crate::physics;
use crate::player;

// Services
pub struct Stats {
    pub time: f32,
    pub level_passed: bool,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            time: 0.0,
            level_passed: false
        }
    }
}

// States
pub struct Pause {
    pub handled: bool,
}

impl Default for Pause {
    fn default() -> Self{
        Self{
            handled: false,
        }
    }
}

pub struct LevelInit {}

pub struct RunLevel {}

pub struct RewindTime {}

// Systems
pub fn after_init(
    mut state: Mut<State>,
    mut game_state: Mut<Stats>,
) {
    *game_state = Stats::default();
    state.clear();
    state.push(RunLevel {});
}

pub fn update (
    mut stats: Mut<Stats>,
    physics_state: Const<physics::State>,
    world: Const<World>,
    mut state: Mut<State>,
) {
    stats.time += physics_state.physics.as_ref()
        .expect("physics_state must be defined").integration_parameters.dt;

    let query = world.query::<(
        &Transform, &player::State,
    )>();

    for (transform, _) in query {
        if transform.translate.x > 210.0 {
            stats.level_passed = true;
            state.push(Pause::default());
        }
    }
}
