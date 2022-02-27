use dotrix::ecs::{ Mut, Const, };
use dotrix::{ Window, Input, State as StateStack, };
use dotrix::math::{ Vec2u, };

use crate::states;
use crate::actions;

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
    mut state: Mut<StateStack>,
    mut window: Mut<Window>,
) {
    if input.is_action_activated(actions::Action::Pause) {
        let paused = if state.get::<states::Pause>().is_none() {
            state.push(states::Pause {});
            true
        } else {
            state.pop_any();
            false
        };

        if let Err(e) = window.set_cursor_grab(!paused) {
            println!("Cannot grab cursor! {}", e);
        }
        window.set_cursor_visible(paused);
    }
}
