use dotrix::{
    Input,
    World,
    ecs::{ Mut, Const, },
};

use crate::physics;
use crate::player;
use crate::actions::Action;

use std::collections::VecDeque;

const STACK_MAX_SIZE: usize = 900;

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
    world: Const<World>,
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

        if stack.rewind_active {
            // physics engine
            println!("{:?} {:?}", stack.index, stack.physics_state.len());
            state.physics = stack.physics_state[stack.index-1].clone();
        }

        // player
        let query = world.query::<(&mut player::State,)>();
        for (state_player,) in query {
            state_player.current_action = Some(
                state_player.action_stack[stack.index-1].clone()
            );

            if stack.rewind_active {
                state_player.override_action = false;
            }
        }
    }
}

pub fn update (
    world: Const<World>,
    mut stack: Mut<Stack>,
    state: Const<physics::State>,
) {
    let index = stack.index;

    // physics engine
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

    // player
    let query = world.query::<(&mut player::State,)>();
    for (state_player,) in query {
        if !stack.rewind_active {
            if index == 0 {
                state_player.action_stack.push_front(
                    state_player.current_action.take()
                        .expect("Current action cannot be None")
                );
            } else {
                state_player.action_stack[index-1] = state_player.current_action
                    .take().expect("Current action cannot be None");
            }
        }

        if state_player.action_stack.len() > STACK_MAX_SIZE {
            state_player.action_stack.pop_back();
        }

        if index == 0 {
            state_player.override_action = false;
        }
    }
}
