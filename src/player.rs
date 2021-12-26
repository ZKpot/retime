use dotrix::{
    Assets, World, Pipeline, Transform, Input,
    pbr::{ Model, Material, },
    ecs::{ Mut, Const, },
    math::{ Vec3, Quat },
};

use crate::actions::Action;

use crate::physics::{
    self,
    vector,
    nalgebra,
};

const TQ_MOVE:   f32 = 25.0;
const TQ_ROTATE: f32 = 5.0;

pub struct Properties {
    pub fwd_angle: f32,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            fwd_angle: 0.0,
        }
    }
}

pub fn startup(
    mut assets: Mut<Assets>,
) {
    assets.import("assets/player.gltf");
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    rigid_body_set: &mut physics::RigidBodySet,
    collider_set: &mut physics::ColliderSet,
) {
    let texture = assets.register("player::texture");
    let mesh = assets.register("player::mesh");

    let rigid_body = physics::RigidBodyBuilder::new_dynamic()
        .translation(physics::vector![0.0, 10.0, 0.0])
        .angular_damping(1.0)
        .build();
    let collider = physics::ColliderBuilder::ball(1.0).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, rigid_body_set);

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
        Properties::default(),
        ball_body_handle,
        Pipeline::default(),
    )));
}

pub fn control(
    world: Const<World>,
    input: Const<Input>,
    mut rigid_body_set: Mut<physics::RigidBodySet>,
) {
    let query = world.query::<(
        &mut Transform, &physics::RigidBodyHandle, &mut Properties,
    )>();

    for (transform, rigid_body, props) in query {

        let body = rigid_body_set.get_mut(*rigid_body).unwrap();
        let position = body.position().translation;
        let rotation = body.position().rotation;

        let fwd_dir = vector![-props.fwd_angle.sin(), 0.0, -props.fwd_angle.cos()];
        let left_dir = vector![-props.fwd_angle.cos(), 0.0, props.fwd_angle.sin()];

        // apply torque
        let mut torque_move = vector![0.0, 0.0, 0.0];
        let mut torque_rotate = vector![0.0, 0.0, 0.0];

        if input.is_action_hold(Action::MoveForward) {
            torque_move = torque_move + fwd_dir;
        }
        if input.is_action_hold(Action::MoveBackward) {
            torque_move = torque_move - fwd_dir;
        }
        if input.is_action_hold(Action::MoveLeft) {
            torque_move = torque_move + left_dir;
        }
        if input.is_action_hold(Action::MoveRight) {
            torque_move = torque_move - left_dir;
        }
        if input.is_action_hold(Action::TurnLeft) {
            torque_rotate += vector![0.0,  1.0, 0.0];
        }
        if input.is_action_hold(Action::TurnRight) {
            torque_rotate += vector![0.0, -1.0, 0.0];
        }

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
