use dotrix::{
    Assets, World, Transform, Input,
    pbr::{ Model, Material, },
    ecs::{ Mut, Const, },
    math::{ Vec3 },
    renderer::Render,
};

use crate::actions;
use crate::time;

use crate::physics::{
    self,
    vector,
    nalgebra,
    Vector,
    Real,
};

use std::f32::consts::PI;
use std::collections::VecDeque;

const TQ_MOVE:   f32 = 25.0;
const TQ_ROTATE: f32 = 5.0;

pub struct State {
    pub fwd_angle: f32,
    pub current_action: Option<Action>,
    pub action_stack: VecDeque<Option<Action>>,
}

impl State {
    pub fn clear_action_stack(&mut self, time_stack: &mut time::Stack) {
        for i in time_stack.index_cleared..time_stack.index {
            self.action_stack[i] = None;
        }

        time_stack.index_cleared = time_stack.index;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            fwd_angle: 0.0,
            current_action: None,
            action_stack: VecDeque::new(),
        }
    }
}

#[derive(Clone)]
pub struct Action {
    pub torque_move: Vector<Real>,
    pub torque_rotate: Vector<Real>,
}

pub fn startup(
    mut assets: Mut<Assets>,
) {
    assets.import("assets/player.gltf");
    assets.import("assets/player.png");
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    physics_state: &mut physics::State,
    player_position: &mut Vec3,
) {
    let state = physics_state.physics.as_mut().expect("physics::State must be defined");

    let texture = assets.register("player::texture");
    let mesh = assets.register("player::mesh");

    let rigid_body = physics::RigidBodyBuilder::new_dynamic()
        .translation(physics::vector![0.0, 10.0, 0.0])
        .angular_damping(1.0)
        .build();
    let collider = physics::ColliderBuilder::ball(1.0).restitution(0.7).build();
    let ball_body_handle = state.rigid_body_set.insert(rigid_body);
    state.collider_set.insert_with_parent(
        collider,
        ball_body_handle,
        &mut state.rigid_body_set
    );

    world.spawn(Some((
        Model::from(mesh),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: *player_position,
            ..Default::default()
        },
        Render::default(),
        time::ActionableObject {
            active: true,
            selected: true,
            is_player: &true,
            tile_texture_name: "player",
        },
        State::default(),
        ball_body_handle,
    )));
}

pub fn control(
    world: Const<World>,
    input: Const<Input>,
    camera: Const<dotrix::Camera>,
    mut physics_state: Mut<physics::State>,
    mut time_stack: Mut<time::Stack>,
) {
    let query = world.query::<(
        &physics::RigidBodyHandle, &mut State, & time::ActionableObject
    )>();

    for (rigid_body, state, object) in query {

        let rigid_body_set = &mut physics_state.physics
            .as_mut().expect("physics::State must be defined")
            .rigid_body_set;

        let body = rigid_body_set.get_mut(*rigid_body).unwrap();

        // align forward direction with the camera view
        state.fwd_angle = PI - camera.pan;

        let fwd_dir = vector![-state.fwd_angle.sin(), 0.0, -state.fwd_angle.cos()];
        let left_dir = vector![-state.fwd_angle.cos(), 0.0, state.fwd_angle.sin()];

        // apply torque
        let mut torque_move = vector![0.0, 0.0, 0.0];
        let mut torque_rotate = vector![0.0, 0.0, 0.0];

        let mut is_any_action = false;

        if object.selected {
            if input.is_action_hold(actions::Action::MoveForward) {
                torque_move = torque_move + fwd_dir;
                is_any_action = true;
            }
            if input.is_action_hold(actions::Action::MoveBackward) {
                torque_move = torque_move - fwd_dir;
                is_any_action = true;
            }
            if input.is_action_hold(actions::Action::MoveLeft) {
                torque_move = torque_move + left_dir;
                is_any_action = true;
            }
            if input.is_action_hold(actions::Action::MoveRight) {
                torque_move = torque_move - left_dir;
                is_any_action = true;
            }
            if input.is_action_hold(actions::Action::TurnLeft) {
                torque_rotate += vector![0.0,  1.0, 0.0];
                is_any_action = true;
            }
            if input.is_action_hold(actions::Action::TurnRight) {
                torque_rotate += vector![0.0, -1.0, 0.0];
                is_any_action = true;
            }
        }

        if is_any_action {
            state.clear_action_stack(&mut time_stack);
        } else {
            if let Some(current_action) = state.current_action.take() {
                torque_move = current_action.torque_move;
                torque_rotate = current_action.torque_rotate;
            }
        }

        state.current_action = Some(Action {
            torque_move,
            torque_rotate,
        });

        body.apply_torque(torque_move*TQ_MOVE, true);
        body.apply_torque(torque_rotate*TQ_ROTATE, true);
    }
}
