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

pub const STACK_MAX_SIZE: usize = 900;

pub struct Stack {
    pub physics_state: VecDeque<Option<physics::PhysicsState>>,
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

    if input.is_action_deactivated(Action::RewindTime) ||
        !input.is_action_hold(Action::RewindTime) ||
        (stack.index >= stack.physics_state.len())
    {
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
    update_stack(
        &mut stack.physics_state,
        physics_state.physics.clone(),
        index
    );

    // player
    let query = world.query::<(&mut player::State,)>();
    for (state_player,) in query {
        update_stack(
            &mut state_player.action_stack,
            state_player.current_action.take(),
            index
        );
    }

    if input.is_action_activated(Action::RewindTime) {
        state_stack.push(states::RewindTime {});
    }
}

fn update_stack<T> (
    stack: &mut VecDeque<Option<T>>,
    new_element: Option<T>,
    index: usize,
) {
    if index == 0 {
        stack.push_front(new_element);
    } else {
        stack[index-1] = new_element;
    }

    while stack.len() > STACK_MAX_SIZE {
        stack.pop_back();
    }
}
