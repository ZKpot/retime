use dotrix::{
    State,
    ecs::{ Mut },
};

pub struct LevelInit {}

pub struct RunLevel {}

pub fn exit_init(
    mut state: Mut<State>,
) {
    state.clear();
    state.push(RunLevel {});
}
