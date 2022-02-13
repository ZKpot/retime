use dotrix::{
    Assets, World, Pipeline, Transform, Input,
    pbr::{ Model, Material, },
    ecs::{ Mut, Const, },
    math::{ Vec3, Quat },
};

use crate::actions;
use std::collections::VecDeque;

use crate::physics::{
    self,
    vector,
    nalgebra,
    Vector,
    Real,
};

use std::f32::consts::PI;
const TQ_MOVE:   f32 = 25.0;
const TQ_ROTATE: f32 = 5.0;

pub struct State {
    pub fwd_angle: f32,
    pub override_action: bool,
    pub current_action: Option<Action>,
    pub action_stack: VecDeque<Action>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            fwd_angle: 0.0,
            override_action: false,
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
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    physics_state: &mut physics::State,
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
            translate: Vec3::new(0.0, 10.0, 0.0),
            ..Default::default()
        },
        State::default(),
        ball_body_handle,
        Pipeline::default(),
    )));
}

pub fn control(
    world: Const<World>,
    input: Const<Input>,
    camera: Const<dotrix::Camera>,
    mut physics_state: Mut<physics::State>,
) {
    let query = world.query::<(
        &mut Transform, &physics::RigidBodyHandle, &mut State,
    )>();

    for (transform, rigid_body, state) in query {

        let rigid_body_set = &mut physics_state.physics
            .as_mut().expect("physics::State must be defined")
            .rigid_body_set;

        let body = rigid_body_set.get_mut(*rigid_body).unwrap();
        let position = body.position().translation;
        let rotation = body.position().rotation;

        // align forward direction with the camera view
        state.fwd_angle = PI - camera.y_angle;

        let fwd_dir = vector![-state.fwd_angle.sin(), 0.0, -state.fwd_angle.cos()];
        let left_dir = vector![-state.fwd_angle.cos(), 0.0, state.fwd_angle.sin()];

        // apply torque
        let mut torque_move = vector![0.0, 0.0, 0.0];
        let mut torque_rotate = vector![0.0, 0.0, 0.0];

        if input.is_action_hold(actions::Action::MoveForward) {
            torque_move = torque_move + fwd_dir;
            state.override_action = true;
        }
        if input.is_action_hold(actions::Action::MoveBackward) {
            torque_move = torque_move - fwd_dir;
            state.override_action = true;
        }
        if input.is_action_hold(actions::Action::MoveLeft) {
            torque_move = torque_move + left_dir;
            state.override_action = true;
        }
        if input.is_action_hold(actions::Action::MoveRight) {
            torque_move = torque_move - left_dir;
            state.override_action = true;
        }
        if input.is_action_hold(actions::Action::TurnLeft) {
            torque_rotate += vector![0.0,  1.0, 0.0];
            state.override_action = true;
        }
        if input.is_action_hold(actions::Action::TurnRight) {
            torque_rotate += vector![0.0, -1.0, 0.0];
            state.override_action = true;
        }

        if !state.override_action {
            if let Some(current_action) = &state.current_action {
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

        // update model transfrom
        transform.translate.x = position.x;
        transform.translate.y = position.y;
        transform.translate.z = position.z;

        transform.rotate = Quat::new(
            rotation.into_inner().w,
            rotation.into_inner().i,
            rotation.into_inner().j,
            rotation.into_inner().k,
        );
    }
}
