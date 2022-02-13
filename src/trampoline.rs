use dotrix::{
    Assets, World,
    pbr,
    math::{ Vec3, },
    ecs::{ self, Mut, Const, },
};

use crate::physics::{ self, vector, nalgebra, };
use crate::player;

const X_INIT: f32 = 72.0;
const Z_INIT: f32 = -11.0;
const MIN_DIST: f32 = 1.4;

pub struct Context {
    initialized: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            initialized: false,
        }
    }
}

pub fn startup(
    mut assets: Mut<Assets>,
) {
    assets.import("assets/trampoline.gltf");
}

pub fn spawn(
    mut context: ecs::Context<Context>,
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut physics_state: Mut<physics::State>,
) {
    let texture = assets.register("trampoline::texture");
    let mesh_id = assets.register("trampoline::mesh");

    if !context.initialized && assets.get(mesh_id).is_some() {

        world.spawn(
            (pbr::solid::Entity {
                mesh: mesh_id,
                texture,
                translate: Vec3::new(X_INIT, 0.0, Z_INIT),
                ..Default::default()
            }).some()
        );

        // add the terrain to the collider set
        let mesh = assets.get(mesh_id).unwrap();

        let mut indices  = Vec::new();

        let vertices = mesh.vertices_as::<[f32; 3]>(0)
            .iter().map(|elem| physics::nalgebra::Point3::new(
                    elem[0],
                    elem[1],
                    elem[2],
                )
            ).collect();

        let indices_mesh  = mesh.indices();

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
        ).translation(vector![X_INIT, 0.0, Z_INIT]).build();

        physics_state.physics.as_mut().expect("physics::State must be defined")
            .collider_set.insert(collider);

        context.initialized = true;
    }
}

pub fn control(
    world: Const<World>,
    mut physics_state: Mut<physics::State>,
) {

    let query = world.query::<(&physics::RigidBodyHandle, &mut player::State,)>();

    for (rigid_body, _) in query {
        let rigid_body_set = &mut physics_state.physics
            .as_mut().expect("physics::State must be defined")
            .rigid_body_set;

        let body = rigid_body_set.get_mut(*rigid_body).unwrap();
        let position = body.position().translation;

        let distance = (
                (position.x - X_INIT).powf(2.0) +
                position.y.powf(2.0) +
                (position.z - Z_INIT).powf(2.0)
            ).sqrt();

        if distance <= MIN_DIST {
            println!("dist to tramp: {:?}", distance);
            body.apply_force(vector![0.0, 6000.0, 0.0], true);
        }
    }
}
