use dotrix::{
    prelude::*,
    Color, World, Input,
    sky::{ skybox, },
    pbr::{ self, Light, },
    ecs::{ Mut, },
    math::Vec3,
    egui, overlay,
    State as StateStack,
};

mod actions;
mod player;
mod ui;
mod camera;
mod physics;
mod level;
mod time;
mod trampoline;
mod states;
mod time_capsule;
mod ui_clock;

fn main() {
    Dotrix::application("ReTime")
        .with(System::from(ui::startup))
        .with(System::from(startup))

        .with(System::from(ui::draw_loading_screen).with(StateStack::on::<states::LoadAssets>()))
        .with(System::from(level::load_assets).with(StateStack::on::<states::LoadAssets>()))

        .with(System::from(ui::draw_menu).with(StateStack::off::<states::LoadAssets>()))
        .with(System::from(ui::draw_in_game_panels).with(StateStack::off::<states::LoadAssets>()))

        .with(System::from(before_init).with(StateStack::on::<states::InitLevel>()))
        .with(System::from(camera::init).with(StateStack::on::<states::InitLevel>()))
        .with(System::from(ui::init).with(StateStack::on::<states::InitLevel>()))
        .with(System::from(level::spawn).with(StateStack::on::<states::InitLevel>()))
        .with(System::from(states::after_init).with(StateStack::on::<states::InitLevel>()))

        .with(System::from(time::rewind).with(StateStack::on::<states::RewindTime>()))
        .with(System::from(time::replay).with(StateStack::on::<states::RunLevel>()))
        .with(System::from(player::control).with(StateStack::on::<states::RunLevel>()))
        .with(System::from(trampoline::control).with(StateStack::on::<states::RunLevel>()))
        .with(
            System::from(states::update)
                .with(StateStack::on::<states::RunLevel>())
                .with(StateStack::on::<states::RewindTime>())
        )
        .with(
            System::from(time_capsule::control)
                .with(StateStack::on::<states::RunLevel>())
                .with(StateStack::on::<states::RewindTime>())
        )
        .with(System::from(time::update_stacks).with(StateStack::on::<states::RunLevel>()))

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
        .with(Service::from(ui::State::default()))
        .with(Service::from(states::Stats::default()))
        .with(Service::from(None as Option<level::Level>))

        .with(pbr::extension)
        .with(skybox::extension)
        .with(overlay::extension)
        .with(egui::extension)

        .run();
}

fn startup(
    mut input: Mut<Input>,
    mut state_stack: Mut<StateStack>,
    mut level: Mut<Option<level::Level>>,
) {
    actions::init_actions(&mut input);

    state_stack.push(states::LoadAssets::default());
    *level = Some(level::Level::from_file("level_1.yaml"));
}

fn before_init(
    mut world: Mut<World>,
    mut physics_state: Mut<physics::State>,
    mut time_stack: Mut<time::Stack>,
    mut camera_state: Mut<camera::State>,
    mut level: Mut<Option<level::Level>>,
) {
    world.reset();
    *physics_state = physics::State::default();
    *time_stack = time::Stack::default();
    *camera_state = camera::State::default();
    *level = Some(level::Level::from_file("level_1.yaml"));

    init_light(&mut world);
}

fn init_light(world: &mut World) {
    world.spawn(Some((Light::Simple {
        position: Vec3::new(20.0, 20.0, 0.0),
        color: Color::white(),
        intensity: 0.5,
        enabled: true,
    },)));
    world.spawn(
        Some((
            Light::Ambient {
                color: Color::white(),
                intensity: 0.2,
            },
        ))
    );
}
