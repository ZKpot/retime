use dotrix::{
    Assets, World, Id,
    assets::Mesh,
    pbr,
    ecs::{ Mut, Const, Context,},
    math::{ Vec3, },
    State as StateStack,
};

use crate::physics;
use crate::time_capsule;
use crate::player;
use crate::states;
use crate::trampoline;

use serde::{Serialize, Deserialize};
use std::{fs, path};

pub struct Ctx {
    loaded: Vec<String>,
    mesh_ids: Vec<Id<Mesh>>,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            loaded: Vec::new(),
            mesh_ids: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct TimeCapsuleInit {
    pub pos: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct PlayerInit {
    pub pos: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct TrampolineInit {
    pub pos_tramp: (f32, f32, f32),
    pub pos_button: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub enum Objects {
    TimeCapsule(TimeCapsuleInit),
    Player(PlayerInit),
    Trampoline(TrampolineInit),
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct Level {
    model: String,
    objects: Vec<Objects>,
}

impl Level {
    pub fn from_file(file_name: &str) -> Self {
        let s = fs::read_to_string(
            path::Path::new(".").join("levels").join(file_name)
        ).unwrap();
        serde_yaml::from_str(&s).unwrap()
    }
}

pub fn load_assets(
    mut ctx: Context<Ctx>,
    mut assets: Mut<Assets>,
    mut state_stack: Mut<StateStack>,
    level_opt: Const<Option<Level>>,
) {
    let mut load_state = state_stack.get_mut::<states::LoadAssets>()
        .expect("something terrible has happened");

    if !load_state.imported {
        let level = level_opt.as_ref().expect("Some level should be loaded");

        let level_path = ["assets/levels/", &level.model, ".gltf"].join("");

        if !ctx.loaded.contains(&level.model) {
            assets.import(&level_path);
            ctx.loaded.push(level.model.clone());
            ctx.mesh_ids.push(assets.register(
                &[&level.model, "mesh"].join("::")
            ));
        }

        for object in level.objects.iter() {
            match object {
                Objects::TimeCapsule(_) => {
                    if !ctx.loaded.contains(&"time_capsule".to_string()) {
                        ctx.mesh_ids.push(time_capsule::load_assets(&mut assets));
                        ctx.loaded.push("time_capsule".to_string());
                    }
                },
                Objects::Player(_) => {
                    if !ctx.loaded.contains(&"player".to_string()) {
                        ctx.mesh_ids.push(player::load_assets(&mut assets));
                        ctx.loaded.push("player".to_string());
                    }
                },
                Objects::Trampoline(_) => {
                    if !ctx.loaded.contains(&"trampoline".to_string()) {
                        ctx.mesh_ids.push(trampoline::load_assets(&mut assets));
                        ctx.loaded.push("trampoline".to_string());
                    }
                },
            }
        }

        load_state.imported = true;
    }

    ctx.mesh_ids.retain(|&x| !assets.get(x).is_some());

    if ctx.mesh_ids.is_empty() {
        state_stack.push(states::InitLevel {});
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

    // spawn all objects
    while let Some(object) = level.objects.pop() {
        match object {
            Objects::Player(init_state) => {
                player::spawn(
                    &mut world,
                    &mut assets,
                    &mut physics_state,
                    &mut Vec3::new(
                        init_state.pos.0,
                        init_state.pos.1,
                        init_state.pos.2
                    )
                );
            },
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
            Objects::Trampoline(init_state) => {
                trampoline::spawn(
                    &mut world,
                    &mut assets,
                    &mut physics_state,
                    &mut Vec3::new(
                        init_state.pos_tramp.0,
                        init_state.pos_tramp.1,
                        init_state.pos_tramp.2
                    ),
                    &mut Vec3::new(
                        init_state.pos_button.0,
                        init_state.pos_button.1,
                        init_state.pos_button.2
                    ),
                )
            },
        }
    }
}
