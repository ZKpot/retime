use dotrix::{
    Assets, World, Transform,
    pbr::{ Model, Material, },
    ecs::{ Mut, },
    math::{ Vec3 },
    renderer::Render,
};

const X: f32 = 45.0;
const Z: f32 = 10.0;
const Y: f32 = 1.1;
const SCALE: f32 = 0.4;

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
            translate: Vec3::new(X, Y, Z),
            scale: Vec3::new(SCALE, SCALE, SCALE),
            ..Default::default()
        },
        Render::default(),
    )));
}
