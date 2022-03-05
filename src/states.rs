use dotrix::{
    State,
    ecs::{ Mut },
};

pub struct Pause {}

pub struct LevelInit {}

pub struct RunLevel {}

pub struct RewindTime {}

pub fn after_init(
    mut state: Mut<State>,
) {
    state.clear();
    state.push(RunLevel {});
}
