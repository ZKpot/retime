use dotrix::{
    prelude::*,
    Assets, CubeMap, Color, World, Pipeline, Input,
    sky::{ skybox, SkyBox, },
    pbr::{ self, Light, },
    ecs::{ Mut, },
};

mod actions;
mod player;
mod settings;
mod camera;
mod physics;
mod terrain;

fn main() {
    Dotrix::application("ReTime")
        .with(System::from(settings::startup))
        .with(System::from(startup))
        .with(System::from(player::startup))
        .with(System::from(camera::startup))
        .with(System::from(terrain::startup))

        .with(System::from(player::control))
        .with(System::from(dotrix::camera::control))
        .with(System::from(camera::control))
        .with(System::from(terrain::spawn))

        .with(Service::from(physics::IslandManager::new()))
        .with(Service::from(physics::BroadPhase::new()))
        .with(Service::from(physics::NarrowPhase::new()))
        .with(Service::from(physics::RigidBodySet::new()))
        .with(Service::from(physics::ColliderSet::new()))
        .with(Service::from(physics::JointSet::new()))
        .with(Service::from(physics::CCDSolver::new()))
        .with(System::from(physics::step))

        .with(pbr::extension)
        .with(skybox::extension)

        .run();
}

fn startup(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut input: Mut<Input>,
    mut rigid_body_set: Mut<physics::RigidBodySet>,
    mut collider_set: Mut<physics::ColliderSet>,
) {
    init_light(&mut world);
    init_skybox(&mut world, &mut assets);
    actions::init_actions(&mut input);

    player::spawn(
        &mut world,
        &mut assets,
        &mut rigid_body_set,
        &mut collider_set,
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
