use std::f32::consts::PI;

use dotrix::{
    Transform, World, Input,
    ecs::{ Mut, Const, Entity, },
    math::{ Vec3, },
};

use crate::actions::Action;
use crate::time;

const DY: f32 = 8.0;
const DZ: f32 = -12.0;

pub struct State {
    position: Option<Vec3>,
    pub player_entity: Option<Entity>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: None,
            player_entity: None,
        }
    }
}

pub fn init (
    mut camera: Mut<dotrix::Camera>,
) {
    camera.pan = PI;
    camera.tilt = PI/8.0;
    camera.position = None;
}

pub fn control (
    mut state: Mut<State>,
    world: Const<World>,
    input: Const<Input>,
    mut camera: Mut<dotrix::Camera>,
) {
    // update camera properties
    if camera.tilt < 0.0 {
        camera.tilt = 0.0;
    };

    let player = world.get::<(
        &mut Transform, &mut time::ActionableObject,
    )>(state.player_entity.expect("Player should be spawned"))
        .take().expect("Player should be spawned");


    camera.target = Vec3::new(
        player.0.translate.x,
        player.0.translate.y,
        player.0.translate.z
    );

    // select next object
    let mut query = world.query::<(
        &mut Transform, &mut time::ActionableObject,
    )>();

    let mut select_next_active_object = false;
    let mut selected_found = false;
    let mut select_player = false;
    let mut object = query.next().expect("At least one object should be spawned");
    let mut next_object = None;

    let reversed = input.is_action_activated(Action::SelectActiveObjectLeft);

    let select = reversed ||
        input.is_action_activated(Action::SelectActiveObjectRight);

    loop {
        let is_selected = object.1.selected;

        if object.1.selected {
            selected_found = true;
            if !object.1.active {
                select_player = true;
                object.1.selected = false;
            } else if select {
                select_next_active_object = true;
                object.1.selected = false;
            }
        }

        if (!next_object.is_some() || selected_found || reversed) && object.1.active && !is_selected {
            if selected_found && reversed && next_object.is_some() {
                break;
            }

            selected_found = false;

            next_object = Some(object);

            if selected_found {
                break;
            }
        }

        match query.next() {
            Some(obj) => object = obj,
            None => break,
        }
    }

    if select_next_active_object || select_player {
        if select_player || !next_object.is_some() {
            next_object = Some(player);
        }

        if let Some(object) = next_object {
            if object.1.is_player {
                state.position = None;
            } else {
                state.position = Some(object.0.translate);
            }

            object.1.selected = true;
        }

        match state.position {
            None => {
                camera.position = None;
            },
            Some(mut camera_position) => {
                camera_position.y += DY;
                camera_position.z += DZ;
                camera.position = Some(camera_position);
            },
        }
    }
}
