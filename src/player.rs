use dotrix::{
    Assets, World, Pipeline, Transform, Camera, Input, Frame,
    pbr::{ Model, Material, },
    ecs::{ Mut, Const, },
    math::{ Vec3, Point3, },
};

use crate::actions::Action;

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
) {
    let texture = assets.register("player::texture");
    let mesh = assets.register("player::mesh");

    world.spawn(Some((
        Model::from(mesh),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        },
        Properties::default(),
        Pipeline::default(),
    )));
}

const SPD: f32 = 5.0;

pub fn control(
    world: Const<World>,
    input: Const<Input>,
    frame: Const<Frame>,
    mut camera: Mut<Camera>,
) {
    let query = world.query::<(
        &mut Transform, &mut Properties
    )>();

    for (transform, _) in query {

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

        camera.target = Point3::new(
            transform.translate.x,
            transform.translate.y,
            transform.translate.z
        );
    }
}
