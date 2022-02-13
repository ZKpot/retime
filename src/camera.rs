use std::f32::consts::PI;

use dotrix::{
    Transform, World, Input,
    ecs::{ Mut, Const, },
    math::{ Vec3, },
};

use crate::player;
use crate::actions::Action;

const CAMERA_SPD: f32 = 0.05; // < 1
const DY: f32 = 8.0;
const DZ: f32 = -12.0;

pub struct Properties {
    pub active: bool,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            active: false,
        }
    }
}

pub struct State {
    control_active: bool,
    pub index: usize,
    position: Option<Vec3>,
    distance: Option<f32>,
    tilt: Option<f32>
}

impl Default for State {
    fn default() -> Self {
        Self {
            control_active: false,
            index: 0,
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

    if input.is_action_activated(Action::MoveCamera) {
        state.index += 1;
    }

    let mut i: usize = 0;

    let query = world.query::<(
        &mut Transform, &mut Properties,
    )>();

    if state.index > 0 {
        for (transform, props) in query {
            i += 1;

            if state.index == i {
                state.position = Some(transform.translate);
                props.active = true;
            } else if props.active {
                props.active = false;
            }
        }
    } else {
        state.position = None;
    }


    if state.index > i {
        state.index = 0;
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
