use dotrix::{
    Assets, World,
    pbr,
    ecs::{ Mut, },
    math::{ Vec3, },
};

use crate::physics;
use crate::time_capsule;
use crate::player;

use serde::{Serialize, Deserialize};
use std::{fs};

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct TimeCapsuleInit {
    pub pos: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub enum Objects {
    TimeCapsule(TimeCapsuleInit),
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct Level {
    model: String,
    player_position: (f32, f32, f32),
    objects: Vec<Objects>,
}

impl Level {
    pub fn from_file(file_name: &str) -> Self {
        let s = fs::read_to_string(&["levels", file_name].join("/")).unwrap();
        serde_yaml::from_str(&s).unwrap()
    }
}

pub fn startup(
    mut assets: Mut<Assets>,
) {
    let level_folder = "assets/levels/";

    for file in fs::read_dir(["./", level_folder].join("")).unwrap() {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        assets.import(&[level_folder, &*file_name].join(""));
    }
}

pub fn spawn (
    mut level_opt: Mut<Option<Level>>,
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut physics_state: Mut<physics::State>,
) {
    let mut level = level_opt.take()
        .expect("Some level should be loaded");

    // spawn level model
    let texture = assets.register(
        &[&*level.model, "texture"].join("::")
    );
    let mesh_id = assets.register(
        &[&*level.model, "mesh"].join("::")
    );

    println!("{}", &[&*level.model, "texture"].join("::"));

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

    let mut indices = Vec::new();

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

    // spawn player
    player::spawn(
        &mut world,
        &mut assets,
        &mut physics_state,
        &mut Vec3::new(
            level.player_position.0,
            level.player_position.1,
            level.player_position.2
        )
    );

    // spawn all objects
    while let Some(object) = level.objects.pop() {
        match object {
            Objects::TimeCapsule(init_state) => {
                time_capsule::spawn(
                    &mut world,
                    &mut assets,
                    &mut Vec3::new(
                        init_state.pos.0,
                        init_state.pos.1,
                        init_state.pos.2
                    )
                )
            },
        }
    }
}
