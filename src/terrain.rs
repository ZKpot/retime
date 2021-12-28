use dotrix::{
    Assets, World,
    pbr,
    ecs::{ self, Mut, },
};

use crate::physics;

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
    assets.import("assets/terrain.gltf");
}

pub fn spawn (
    mut context: ecs::Context<Context>,
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut collider_set: Mut<physics::ColliderSet>,
) {
    let texture = assets.register("terrain::texture");
    let mesh_id = assets.register("terrain::mesh");

    if !context.initialized && assets.get(mesh_id).is_some() {
        world.spawn(
            (pbr::solid::Entity {
                mesh: mesh_id,
                texture,
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
        ).build();
        collider_set.insert(collider);

        context.initialized = true;
    }
}
