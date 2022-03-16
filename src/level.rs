use dotrix::{
    Assets, World,
    pbr,
    ecs::{ Mut, },
};

use crate::physics;

pub fn startup(
    mut assets: Mut<Assets>,
) {
    assets.import("assets/level.gltf");
}

pub fn spawn (
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut physics_state: Mut<physics::State>,
) {
    let texture = assets.register("level::texture");
    let mesh_id = assets.register("level::mesh");

    while !assets.get(mesh_id).is_some() {}

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
        .collect::<Vec<_>>().iter()
        .map(|elem| physics::nalgebra::Point3::new(
                elem[0],
                elem[1],
                elem[2],
            )
        ).collect();

    let indices_mesh = mesh.indices().take()
        .expect("terrain mesh should contain indices");

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

    physics_state.physics.as_mut().expect("physics::State must be defined")
        .collider_set.insert(collider);
}
