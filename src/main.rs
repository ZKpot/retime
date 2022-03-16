use dotrix::{
    prelude::*,
    Color, World, Input,
    sky::{ skybox, },
    pbr::{ self, Light, },
    ecs::{ Mut, },
    egui, overlay,
    State as StateStack,
};

mod actions;
mod player;
mod ui;
mod camera;
mod physics;
mod terrain;
mod time;
mod trampoline;
mod states;

fn main() {
    Dotrix::application("ReTime")
        .with(System::from(ui::startup))
        .with(System::from(startup))
        .with(System::from(player::startup))
        .with(System::from(camera::startup))
        .with(System::from(terrain::startup))
        .with(System::from(trampoline::startup))

        .with(System::from(ui::draw_panel))
        .with(System::from(ui::draw_menu))

        .with(
            System::from(before_init).with(StateStack::on::<states::LevelInit>())
        )
        .with(
            System::from(ui::init).with(StateStack::on::<states::LevelInit>())
        )
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
            System::from(states::after_init).with(StateStack::on::<states::LevelInit>())
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
        .with(
            System::from(states::update)
                .with(StateStack::on::<states::RunLevel>())
                .with(StateStack::on::<states::RewindTime>())
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
        .with(Service::from(ui::State::default()))
        .with(Service::from(states::Stats::default()))

        .with(pbr::extension)
        .with(skybox::extension)
        .with(overlay::extension)
        .with(egui::extension)

        .run();
}

fn startup(
    mut input: Mut<Input>,
    mut state_stack: Mut<StateStack>,
) {
    actions::init_actions(&mut input);

    state_stack.push(states::LevelInit {});
}

fn before_init(
    mut world: Mut<World>,
    mut physics_state: Mut<physics::State>,
    mut time_stack: Mut<time::Stack>,
) {
    world.reset();
    *physics_state = physics::State::default();
    *time_stack = time::Stack::default();

    init_light(&mut world);
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
