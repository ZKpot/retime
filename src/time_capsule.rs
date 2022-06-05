use dotrix::{
    Assets, World, Transform, Id,
    assets::Mesh,
    pbr::{ Model, Material, },
    ecs::{ Mut, Entity, },
    math::{ Vec3, Quat, InnerSpace, },
    renderer::Render,
};

use crate::player;
use crate::time;

const SCALE: f32 = 0.4;
const MIN_DIST: f32 = 0.75;

pub struct State {
    position: Vec3,
}

pub fn load_assets(
    assets: &mut Assets,
) -> Id<Mesh> {
    assets.import("assets/time_capsule.gltf");
    assets.register("time_capsule::mesh")
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    position: Vec3,
) {
    let texture = assets.register("time_capsule::texture");
    let mesh = assets.register("time_capsule::mesh");

    world.spawn(Some((
        Model::from(mesh),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            scale: Vec3::new(SCALE, SCALE, SCALE),
            ..Default::default()
        },
        State{ position },
        Render::default(),
    )));
}

pub fn control(
    mut world: Mut<World>,
    mut time_stack: Mut<time::Stack>,
) {
    // player
    let mut player_x = 0.0;
    let mut player_y = 0.0;
    let mut player_z = 0.0;
    let query = world.query::<(&player::State, &Transform)>();
    for (_, transform) in query {
        player_x = transform.translate.x;
        player_y = transform.translate.y;
        player_z = transform.translate.z;
    }

    // time capsule
    let mut to_exile = Vec::new();

    let query = world.query::<(&Entity, &mut State, &mut Transform)>();

    for (entity, state, transform) in query {
        // simple animation
        let theta: f32 = 0.02;
        let q = Quat::from_sv(
            (theta/2.0).cos(),
            Vec3::new(0.1, 0.95, 0.2).normalize() * (theta/2.0).sin(),
        );

        transform.rotate = transform.rotate * q;

        transform.translate = state.position;

        let dist_to_capsule = (
            (player_x-state.position.x).powf(2.0)+
            (player_y-state.position.y).powf(2.0)+
            (player_z-state.position.z).powf(2.0)
        ).sqrt();

        if (dist_to_capsule<=MIN_DIST) && (time_stack.index_max<time::STACK_MAX_SIZE) {
            time_stack.index_max += time::STACK_MAX_SIZE / 2;
            to_exile.push(*entity);
        }
    }

    for i in 0..to_exile.len() {
        world.exile(to_exile[i]);
    }
}
