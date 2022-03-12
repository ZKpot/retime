use std::f32::consts::PI;

use dotrix::{
    Transform, World, Input,
    ecs::{ Mut, Const, },
    math::{ Vec3, },
};

use crate::player;
use crate::actions::Action;
use crate::time;

const CAMERA_SPD: f32 = 0.05; // < 1
const DY: f32 = 8.0;
const DZ: f32 = -12.0;

pub struct State {
    control_active: bool,
    position: Option<Vec3>,
    distance: Option<f32>,
    tilt: Option<f32>
}

impl Default for State {
    fn default() -> Self {
        Self {
            control_active: false,
            position: None,
            distance: None,
            tilt: None,
        }
    }
}

pub fn startup (
    mut camera: Mut<dotrix::Camera>,
) {
    camera.pan = PI;
    camera.tilt = PI/8.0;
    camera.distance = 10.0;
}

pub fn control (
    mut state: Mut<State>,
    world: Const<World>,
    input: Const<Input>,
    mut camera: Mut<dotrix::Camera>,
) {
    if camera.tilt < 0.0 {
        camera.tilt = 0.0;
    };

    // make camera follow the player
    let query = world.query::<(
        &mut Transform, &mut player::State,
    )>();

    for (transform, _) in query {
        // update camera properties
        camera.target = Vec3::new(
            transform.translate.x,
            transform.translate.y,
            transform.translate.z
        );
    }

    let query = world.query::<(
        &mut time::ActionableObject,
    )>();

    let mut select_next_active_object = false;

    for (object,) in query {
        if !object.active && object.selected {
            select_next_active_object = true;
        }
    }

    if input.is_action_activated(Action::MoveCamera) || select_next_active_object {
        let query = world.query::<(
            &mut Transform, &mut time::ActionableObject,
        )>();

        let mut selected_found = false;
        let mut new_object_selected = false;

        for (transform, object) in query {
            if object.active && selected_found {
                if *object.is_player {
                    state.position = None;
                } else {
                    state.position = Some(transform.translate);
                }
                object.selected = true;
                new_object_selected = true;
            }

            if object.selected && !new_object_selected {
                selected_found = true;
                object.selected = false;
            }
        }

        let query = world.query::<(
            &mut Transform, &mut time::ActionableObject,
        )>();

        if !new_object_selected {
            for (transform, object) in query {
                if object.active {
                    if *object.is_player {
                        state.position = None;
                    } else {
                        state.position = Some(transform.translate);
                    }
                    object.selected = true;
                    break;
                }
            }
        }
    }

    match state.position {
        None => {
            if let Some(distance) = state.distance.take() {
                camera.distance = distance;
            }

            if let Some(tilt) = state.tilt.take() {
                camera.tilt = tilt;
            }

            if input.is_action_deactivated(Action::RotateCamera) {
                state.control_active = true;
            }

            if !input.is_action_hold(Action::RotateCamera) {
                camera.pan = if state.control_active {
                    let mut pan_error = PI - camera.pan;

                    if pan_error.abs() > PI {
                        pan_error = -pan_error.signum()*PI + pan_error%PI;
                    }

                    if pan_error.abs() < PI/128.0 {
                        state.control_active = false;
                        PI
                    } else {
                        camera.pan + pan_error * CAMERA_SPD
                    }
                } else {
                    PI
                }
            }
            camera.position = None;
        },
        Some(mut camera_position) => {
            camera_position.y += DY;
            camera_position.z += DZ;
            camera.position = Some(camera_position);
        },
    }
}
