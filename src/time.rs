use dotrix::{
    Input,
    World,
    ecs::{ Mut, Const, },
    State as StateStack,
};

use crate::physics;
use crate::player;
use crate::actions::Action;
use crate::states;

use std::collections::VecDeque;

const STACK_MAX_SIZE: usize = 900;

pub struct Stack {
    physics_state: VecDeque<Option<physics::PhysicsState>>,
    pub index: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            physics_state: VecDeque::new(),
            index: 0,
        }
    }
}

pub fn rewind (
    mut stack: Mut<Stack>,
    mut physics_state: Mut<physics::State>,
    input: Const<Input>,
    mut state_stack: Mut<StateStack>,
) {
    stack.index += 1;

    println!("{:?} {:?}", stack.index, stack.physics_state.len());
    physics_state.physics = stack.physics_state[stack.index-1].clone();

    if input.is_action_deactivated(Action::RewindTime) || (stack.index >= stack.physics_state.len()) {
        state_stack.pop::<states::RewindTime>().expect("Expected RewindTime state");
    }
}

pub fn replay (
    world: Const<World>,
    mut stack: Mut<Stack>,
) {
    if stack.index > 0 {
        stack.index -= 1;
        println!("{:?} {:?}", stack.index, stack.physics_state.len());

        // player
        if stack.index > 0 {
            let query = world.query::<(&mut player::State,)>();
            for (state_player,) in query {
                state_player.current_action =
                    state_player.action_stack[stack.index-1].clone();
            }
        }
    }
}

pub fn update_stacks (
    world: Const<World>,
    mut stack: Mut<Stack>,
    physics_state: Const<physics::State>,
    input: Const<Input>,
    mut state_stack: Mut<StateStack>,
) {
    let index = stack.index;

    // physics engine
    if index == 0 {
        stack.physics_state.push_front(physics_state.physics.clone());
    } else {
        stack.physics_state[index-1] = physics_state.physics.clone();
    }

    while stack.physics_state.len() > STACK_MAX_SIZE {
        stack.physics_state.pop_back();
    }

    // player
    let query = world.query::<(&mut player::State,)>();
    for (state_player,) in query {
        if index == 0 {
            state_player.action_stack.push_front(state_player.current_action.take());
        } else {
            state_player.action_stack[index-1] = state_player.current_action.take();
        }

        while state_player.action_stack.len() > STACK_MAX_SIZE {
            state_player.action_stack.pop_back();
        }
    }

    if input.is_action_activated(Action::RewindTime) {
        state_stack.push(states::RewindTime {});
    }
}
