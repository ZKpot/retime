use dotrix::{
    Assets, World, Pipeline, Transform, Camera, Input, Frame,
    pbr::{ Model, Material, },
    ecs::{ Mut, Const, },
    math::{ Vec3, Point3, },
};

use crate::actions::Action;

use crate::physics::{
    self,
    nalgebra,
};

pub struct Properties {
}

impl Default for Properties {
    fn default() -> Self {
        Self {
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

const SPD: f32 = 5.0;

pub fn control(
    world: Const<World>,
    input: Const<Input>,
    frame: Const<Frame>,
    mut camera: Mut<Camera>,
    mut rigid_body_set: Mut<physics::RigidBodySet>,
) {
    let query = world.query::<(
        &mut Transform, &physics::RigidBodyHandle
    )>();

    for (transform, rigid_body) in query {

        let body = rigid_body_set.get_mut(*rigid_body).unwrap();
        let position = body.position().translation;

        transform.translate.x = position.x;
        transform.translate.y = position.y;
        transform.translate.z = position.z;

        let dt = frame.delta().as_secs_f32();

        if input.is_action_hold(Action::MoveForward) {
            transform.translate.x = transform.translate.x + SPD * dt;
        }
        if input.is_action_hold(Action::MoveBackward) {
            transform.translate.x = transform.translate.x - SPD * dt;
        }
        if input.is_action_hold(Action::MoveLeft) {
            transform.translate.z = transform.translate.z - SPD * dt;
        }
        if input.is_action_hold(Action::MoveRight) {
            transform.translate.z = transform.translate.z + SPD * dt;
        }

        body.set_translation(
            physics::vector![
                transform.translate.x,
                transform.translate.y,
                transform.translate.z
            ],
            true,
        );

        camera.target = Point3::new(
            transform.translate.x,
            transform.translate.y,
            transform.translate.z
        );
    }
}
