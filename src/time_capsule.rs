use dotrix::{
    Assets, World, Transform,
    pbr::{ Model, Material, },
    ecs::{ Mut, Entity, },
    math::{ Vec3 },
    renderer::Render,
};

use crate::player;
use crate::time;

const SCALE: f32 = 0.4;
const MIN_DIST: f32 = 0.75;

pub struct State {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub collected: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            x: 45.0,
            z: 10.0,
            y: 1.1,
            collected: false,
        }
    }
}

pub fn startup(
    mut assets: Mut<Assets>,
) {
    assets.import("assets/time_capsule.gltf");
}

pub fn spawn(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
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
        State::default(),
        Render::default(),
    )));
}

pub fn control(
    mut world: Mut<World>,
    mut time_stack: Mut<time::Stack>,
) {
    // player
    let mut player_x = 0.0;
    let mut player_z = 0.0;
    let query = world.query::<(&player::State, &Transform)>();
    for (_, transform) in query {
        player_x = transform.translate.x;
        player_z = transform.translate.z;
    }

    // time capsule
    let mut to_exile = Vec::new();

    let query = world.query::<(&Entity, &mut State, &mut Transform)>();

    for (entity, state, transform) in query {
        transform.translate.x = state.x;
        transform.translate.y = state.y;
        transform.translate.z = state.z;


        let dist_to_capsule = ((player_x-state.x).powf(2.0)+(player_z-state.z).powf(2.0)).sqrt();

        if dist_to_capsule <= MIN_DIST {
            state.collected = true;
            time_stack.index_max += time::STACK_MAX_SIZE;
        }

        if state.collected {
            to_exile.push(*entity);
        }
    }

    for i in 0..to_exile.len() {
        world.exile(to_exile[i]);
    }
}
