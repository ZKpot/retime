use dotrix::{
    Input,
    ecs::{ Mut, Const, },
};

use crate::physics;
use crate::actions::Action;

use std::collections::VecDeque;

const STACK_MAX_SIZE: usize = 600;

pub struct Stack {
    physics_state: VecDeque<Option<physics::PhysicsState>>,
    index: usize,
    rewind_active: bool,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            physics_state: VecDeque::new(),
            index: 0,
            rewind_active: false,
        }
    }
}

pub fn rewind (
    mut stack: Mut<Stack>,
    mut state: Mut<physics::State>,
    input: Const<Input>,
) {
    if input.is_action_activated(Action::RewindTime) {
        stack.rewind_active = true;
    }

    if input.is_action_deactivated(Action::RewindTime) ||
        (stack.index >= stack.physics_state.len())
    {
        stack.rewind_active = false;
    }

    if stack.rewind_active {
        stack.index += 1;
    } else if stack.index > 0 {
        stack.index -= 1;
    }

    if stack.index > 0 {
        println!("{:?} {:?}", stack.index, stack.physics_state.len());
        state.physics = stack.physics_state[stack.index-1].clone();
    }
}

pub fn update (
    mut stack: Mut<Stack>,
    state: Const<physics::State>,
) {
    let index = stack.index;

    if !stack.rewind_active {
        if index == 0 {
            stack.physics_state.push_front(state.physics.clone());
        } else {
            stack.physics_state[index-1] = state.physics.clone();
        }
    }

    if stack.physics_state.len() > STACK_MAX_SIZE {
        stack.physics_state.pop_back();
    }
}
