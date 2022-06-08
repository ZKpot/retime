use std::f32::consts::PI;
use std::sync::{ Arc, Mutex, };

use dotrix::{
    Assets, World, Transform, Input, Id,
    assets::Mesh,
    pbr::{ Model, Material, },
    math::{ Vec3, },
    ecs::{ Mut, Const, },
    math::{ Quat },
    renderer::Render,
};

use crate::physics::{ self, vector, nalgebra, };
use crate::player;
use crate::actions::Action;
use crate::time;

const TRAMP_MIN_DIST: f32 = 1.75;
const BUTTON_MIN_DIST: f32 = 1.5;

pub struct State {
    base_position: Vec3,
    button_position: Vec3,
    active: bool,
}

pub fn load_assets(
    assets: &mut Assets,
) -> Id<Mesh> {
    assets.import("assets/trampoline.gltf");
    assets.import("assets/trampoline.png");
    assets.register("trampoline::mesh")
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    physics_state: &mut physics::State,
    base_position: Vec3,
    button_position: Vec3,
) {
    let texture = assets.register("trampoline::texture");
    let mesh_id = assets.register("trampoline::mesh");

    let state = Arc::new(Mutex::new(State {
        base_position,
        button_position,
        active: false,
    }));

    // spawn trampoline
    world.spawn(Some((
        Model::from(mesh_id),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: base_position,
            rotate: Quat::new((PI/2.0).cos(), 0.0, 0.0, (PI/2.0).sin()),
            scale: Vec3::new(0.4, 1.0, 0.4)
        },
        Render::default(),
        state.clone(),
        time::ActionableObject {
            active: false,
            selected: false,
            is_player: false,
            tile_texture_name: "trampoline",
        },
    )));

    // add trampoline the collider set
    let mesh = assets.get(mesh_id).unwrap();

    let mut indices = Vec::new();

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
    ).translation(vector![base_position.x, 0.0, base_position.z]).build();

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
            translate: button_position,
            rotate: Quat::new((PI/4.0).cos(), 0.0, 0.0, (PI/4.0).sin()),
            scale: Vec3::new(0.4, 1.0, 0.4)
        },
        Render::default(),
        state.clone(),
    )));
}

pub fn control(
    world: Const<World>,
    input: Const<Input>,
    mut physics_state: Mut<physics::State>,
) {


    // query player
    let mut query = world.query::<(&physics::RigidBodyHandle, &mut player::State,)>();

    let (rigid_body, _) = query.next().take().expect("player is not found");

    let rigid_body_set = &mut physics_state.physics
            .as_mut().expect("physics::State must be defined")
            .rigid_body_set;

    let body = rigid_body_set.get_mut(*rigid_body).unwrap();
    let player_position = body.position().translation;

    // query button
    let query = world.query::<(&mut Transform, &mut Arc<Mutex<State>>,)>();

    for (transform, state) in query {
        let mut state = state.lock().unwrap();
        if !state.active {
            let distance_to_button = (
                (player_position.x - state.button_position.x).powf(2.0) +
                (player_position.y - state.button_position.y).powf(2.0) +
                (player_position.z - state.button_position.z).powf(2.0)
            ).sqrt();

            if distance_to_button <= BUTTON_MIN_DIST {
                state.active = true;
            }
        }

        if state.active {
            transform.rotate = Quat::new((3.0*PI/4.0).cos(), 0.0, 0.0, (3.0*PI/4.0).sin());
        } else {
            transform.rotate = Quat::new((PI/4.0).cos(), 0.0, 0.0, (PI/4.0).sin());
        };
    }


    // query trampoline
    let query = world.query::<(
        &mut Transform,
        &mut Arc<Mutex<State>>,
        &mut time::ActionableObject,
    )>();

    for (transform, state, object) in query {
        let mut state = state.lock().unwrap();
        if state.active {
            let distance_to_tramp = (
                (player_position.x - state.base_position.x).powf(2.0) +
                (player_position.y - state.base_position.y).powf(2.0) +
                (player_position.z - state.base_position.z).powf(2.0)
            ).sqrt();

            if (distance_to_tramp <= TRAMP_MIN_DIST) &&
                input.is_action_activated(Action::TurnRight) &&
                object.selected
            {
                println!("dist to tramp: {:?}", distance_to_tramp);
                body.apply_impulse(vector![0.0, 90.0, 0.0], true);
                state.active = false;
            }
        }

        if state.active {
            transform.rotate = Quat::new(PI.cos(), 0.0, 0.0, PI.sin());
        } else {
            transform.rotate = Quat::new((PI/2.0).cos(), 0.0, 0.0, (PI/2.0).sin());
        };

        object.active = state.active;
    }
}
