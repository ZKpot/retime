use dotrix::{
    prelude::*,
    Assets, CubeMap, Color, World, Input,
    sky::{ skybox, SkyBox, },
    pbr::{ self, Light, },
    ecs::{ Mut, },
    renderer::Render,
};

mod actions;
mod player;
mod settings;
mod camera;
mod physics;
mod terrain;
mod time;
mod trampoline;

fn main() {
    Dotrix::application("ReTime")
        .with(System::from(settings::startup))
        .with(System::from(startup))
        .with(System::from(player::startup))
        .with(System::from(camera::startup))
        .with(System::from(terrain::startup))
        .with(System::from(trampoline::startup))

        .with(System::from(terrain::spawn))
        .with(System::from(trampoline::spawn))

        .with(System::from(time::rewind))
        .with(System::from(player::control))
        .with(System::from(trampoline::control))
        .with(System::from(dotrix::camera::control))
        .with(System::from(camera::control))
        .with(System::from(time::update))

        .with(Service::from(physics::State::default()))
        .with(Service::from(time::Stack::default()))
        .with(Service::from(camera::State::default()))
        .with(System::from(physics::step))

        .with(pbr::extension)
        .with(skybox::extension)

        .run();
}

fn startup(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut input: Mut<Input>,
    mut physics_state: Mut<physics::State>,
) {
    init_light(&mut world);
    init_skybox(&mut world, &mut assets);
    actions::init_actions(&mut input);

    player::spawn(
        &mut world,
        &mut assets,
        &mut physics_state,
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
        Render::default(),
    )));
}
