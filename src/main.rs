use dotrix::{
    prelude::*,
    Assets, CubeMap, Color, World, Input,
    sky::{ skybox, SkyBox, },
    pbr::{ self, Light, },
    ecs::{ Mut, },
    renderer::Render,
    State as StateStack,
};

mod actions;
mod player;
mod settings;
mod camera;
mod physics;
mod terrain;
mod time;
mod trampoline;
mod states;

fn main() {
    Dotrix::application("ReTime")
        .with(System::from(settings::startup))
        .with(System::from(startup))
        .with(System::from(player::startup))
        .with(System::from(camera::startup))
        .with(System::from(terrain::startup))
        .with(System::from(trampoline::startup))

        .with(System::from(settings::menu))
        .with(
            System::from(terrain::spawn).with(StateStack::on::<states::LevelInit>())
        )
        .with(
            System::from(trampoline::spawn).with(StateStack::on::<states::LevelInit>())
        )
        .with(
            System::from(player::spawn).with(StateStack::on::<states::LevelInit>())
        )
        .with(
            System::from(states::exit_init).with(StateStack::on::<states::LevelInit>())
        )

        .with(
            System::from(time::rewind).with(StateStack::on::<states::RewindTime>())
        )
        .with(
            System::from(time::replay).with(StateStack::on::<states::RunLevel>())
        )
        .with(
            System::from(player::control).with(StateStack::on::<states::RunLevel>())
        )
        .with(
            System::from(trampoline::control).with(StateStack::on::<states::RunLevel>())
        )

        .with(
            System::from(time::update_stacks)
                .with(StateStack::on::<states::RunLevel>())
        )

        .with(System::from(physics::update_models))
        .with(System::from(physics::step).with(StateStack::on::<states::RunLevel>()))
        .with(
            System::from(dotrix::camera::control)
                .with(StateStack::on::<states::RunLevel>())
                .with(StateStack::on::<states::RewindTime>())
        )
        .with(
            System::from(camera::control)
                .with(StateStack::on::<states::RunLevel>())
                .with(StateStack::on::<states::RewindTime>())
        )

        .with(Service::from(physics::State::default()))
        .with(Service::from(time::Stack::default()))
        .with(Service::from(camera::State::default()))

        .with(pbr::extension)
        .with(skybox::extension)

        .run();
}

fn startup(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut input: Mut<Input>,
    mut state_stack: Mut<StateStack>,
) {
    init_light(&mut world);
    init_skybox(&mut world, &mut assets);
    actions::init_actions(&mut input);

    state_stack.push(states::LevelInit {});
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
