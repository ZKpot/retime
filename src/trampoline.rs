use std::f32::consts::PI;

use dotrix::{
    Assets, World, Transform, Input,
    pbr::{ Model, Material, },
    math::{ Vec3, },
    ecs::{ self, Mut, Const, },
    math::{ Quat },
    renderer::Render,
};

use crate::physics::{ self, vector, nalgebra, };
use crate::player;
use crate::camera;
use crate::actions::Action;

const TRAMP_X: f32 = 72.0;
const TRAMP_Y: f32 = 0.01;
const TRAMP_Z: f32 = -11.0;
const TRAMP_MIN_DIST: f32 = 1.4;

const BUTTON_X: f32 = 174.15;
const BUTTON_Y: f32 = -20.0;
const BUTTON_Z: f32 = -11.0;
const BUTTON_MIN_DIST: f32 = 1.4;

pub struct Context {
    active: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            active: false,
        }
    }
}

pub struct State {
}

impl Default for State {
    fn default() -> Self {
        Self {
        }
    }
}

pub fn startup(
    mut assets: Mut<Assets>,
) {
    assets.import("assets/trampoline.gltf");
    assets.import("assets/trampoline.png");
}

pub fn spawn(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut physics_state: Mut<physics::State>,
) {
    let texture = assets.register("trampoline::texture");
    let mesh_id = assets.register("trampoline::mesh");

    while !assets.get(mesh_id).is_some() {}

    // spawn trampoline
    world.spawn(Some((
        Model::from(mesh_id),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: Vec3::new(TRAMP_X, TRAMP_Y, TRAMP_Z),
            rotate: Quat::new((PI/2.0).cos(), 0.0, 0.0, (PI/2.0).sin()),
            ..Default::default()
        },
        Render::default(),
        State::default(),
        camera::Properties::default(),
    )));

    // add trampoline the collider set
    let mesh = assets.get(mesh_id).unwrap();

    let mut indices  = Vec::new();

    let vertices = mesh.vertices_as::<[f32; 3]>(0).collect::<Vec<_>>()
        .iter().map(|elem| physics::nalgebra::Point3::new(
                elem[0],
                elem[1],
                elem[2],
            )
        ).collect();

    let indices_mesh = mesh.indices().take()
    .expect("trampoline mesh should contain indices");

    for i in 0..indices_mesh.len()/3 {
        indices.push([
            indices_mesh[i*3],
            indices_mesh[i*3+1],
            indices_mesh[i*3+2],
        ]);
    }

    let collider = physics::ColliderBuilder::trimesh(
        vertices,
        indices,
    ).translation(vector![TRAMP_X, 0.0, TRAMP_Z]).build();

    physics_state.physics.as_mut().expect("physics::State must be defined")
        .collider_set.insert(collider);

    // add activation button
    world.spawn(Some((
        Model::from(mesh_id),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: Vec3::new(BUTTON_X, BUTTON_Y, BUTTON_Z),
            rotate: Quat::new((PI/4.0).cos(), 0.0, 0.0, (PI/4.0).sin()),
            scale: Vec3::new(0.4, 1.0, 0.4)
        },
        Render::default(),
        State::default(),
    )));
}

pub fn control(
    mut context: ecs::Context<Context>,
    world: Const<World>,
    input: Const<Input>,
    mut physics_state: Mut<physics::State>,
) {
    let mut state_changed = false;

    // query trampoline
    let query = world.query::<(&camera::Properties, )>();

    let mut camera_active = false;

    for (camera_prop,) in query {
        camera_active = camera_prop.active;
    }

    // query player
    let query = world.query::<(
        &physics::RigidBodyHandle,
        &mut player::State,
    )>();

    for (rigid_body, _) in query {
        let rigid_body_set = &mut physics_state.physics
            .as_mut().expect("physics::State must be defined")
            .rigid_body_set;

        let body = rigid_body_set.get_mut(*rigid_body).unwrap();
        let position = body.position().translation;

        if context.active {
            let distance_to_tramp = (
                (position.x - TRAMP_X).powf(2.0) +
                position.y.powf(2.0) +
                (position.z - TRAMP_Z).powf(2.0)
            ).sqrt();

            if (distance_to_tramp <= TRAMP_MIN_DIST) &&
                input.is_action_activated(Action::TurnRight) &&
                camera_active
            {
                println!("dist to tramp: {:?}", distance_to_tramp);
                body.apply_impulse(vector![0.0, 150.0, 0.0], true);
                context.active = false;
                state_changed = true;
            }
        } else {
            let distance_to_button = (
                (position.x - BUTTON_X).powf(2.0) +
                (position.y - BUTTON_Y).powf(2.0) +
                (position.z - BUTTON_Z).powf(2.0)
            ).sqrt();

            if distance_to_button <= BUTTON_MIN_DIST {
                println!("dist to button: {:?}", distance_to_button);
                context.active = true;
                state_changed = true;
            }
        }
    }

    //query trampoline and button
    if state_changed {
        let query = world.query::<(
            &mut Transform,
            &State,
        )>();

        for (transform, _) in query {
            transform.rotate = transform.rotate *
                Quat::new((PI/2.0).cos(), 0.0, 0.0, (PI/2.0).sin());
        }
    }
}
