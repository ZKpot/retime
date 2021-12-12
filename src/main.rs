use std::f32::consts::PI;

use dotrix::{
    prelude::*,
    Assets, Camera, CubeMap, Color, World, Pipeline,
    assets::Mesh,
    sky::{ skybox, SkyBox, },
    pbr::{ self, Light, },
    camera,
    math::{ Point3, },
    ecs::{ Mut, },
};

fn main() {
    Dotrix::application("ReTime")
        .with(System::from(startup))
        .with(System::from(camera::control))
        .with(pbr::extension)
        .with(skybox::extension)
        .run();
}

fn startup(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut camera: Mut<Camera>,
) {
    init_terrain(&mut world, &mut assets);
    init_light(&mut world);
    init_camera(&mut camera);
    init_skybox(&mut world, &mut assets);
}

fn init_terrain(
    world: &mut World,
    assets: &mut Assets,
) {
    let size = 25.0;
    let mut positions = Vec::new();
    let mut uvs = Vec::new();

    positions.push([-size, 0.0, -size]);
    positions.push([-size, 0.0,  size]);
    positions.push([ size, 0.0, -size]);
    positions.push([ size, 0.0, -size]);
    positions.push([-size, 0.0,  size]);
    positions.push([ size, 0.0,  size]);

    uvs.push([ 0.0,  0.0]);
    uvs.push([ 0.0, size]);
    uvs.push([size,  0.0]);
    uvs.push([size,  0.0]);
    uvs.push([ 0.0, size]);
    uvs.push([size, size]);

    let normals = Mesh::calculate_normals(&positions, None);

    let mut mesh = Mesh::default();

    mesh.with_vertices(&positions);
    mesh.with_vertices(&normals);
    mesh.with_vertices(&uvs);

    // Store mesh and get its ID
    let mesh = assets.store(mesh);

    // import terrain texture and get its ID
    assets.import("assets/terrain.png");
    let texture = assets.register("terrain");

    world.spawn(
        (pbr::solid::Entity {
            mesh,
            texture,
            ..Default::default()
        }).some()
    );
}

fn init_light(world: &mut World) {
    world.spawn(
        Some((
            Light::Ambient {
                color: Color::white(),
                intensity: 0.5,
            },
        ))
    );
}

fn init_camera(camera: &mut Camera) {
    camera.target = Point3::new(0.0, 2.0, 0.0);
    camera.y_angle = PI;
    camera.xz_angle = PI/8.0;
    camera.distance = 10.0;
}

fn init_skybox(
    world: &mut World,
    assets: &mut Assets,
) {
    assets.import("assets/skybox/skybox_right.png");
    assets.import("assets/skybox/skybox_left.png");
    assets.import("assets/skybox/skybox_top.png");
    assets.import("assets/skybox/skybox_bottom.png");
    assets.import("assets/skybox/skybox_front.png");
    assets.import("assets/skybox/skybox_back.png");

    world.spawn(Some((
        SkyBox {
            view_range: 500.0,
            ..Default::default()
        },
        CubeMap {
            right: assets.register("skybox_right"),
            left: assets.register("skybox_left"),
            top: assets.register("skybox_top"),
            bottom: assets.register("skybox_bottom"),
            back: assets.register("skybox_back"),
            front: assets.register("skybox_front"),
            ..Default::default()
        },
        Pipeline::default()
    )));
}
