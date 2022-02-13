use std::f32::consts::PI;

use dotrix::{
    Transform, World, Input,
    ecs::{ Mut, Const, Context, },
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

pub struct Camera {
    control_active: bool,
    index: usize,
    position: Option<Vec3>,
    distance: Option<f32>,
    tilt: Option<f32>
}

impl Default for Camera {
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
    mut context: Context<Camera>,
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
        context.index += 1;
    }

    let mut i: usize = 0;

    let query = world.query::<(
        &mut Transform, &mut Properties,
    )>();

    if context.index > 0 {
        for (transform, props) in query {
            i += 1;

            if context.index == i {
                context.position = Some(transform.translate);
                props.active = true;
            } else if props.active {
                props.active = false;
            }
        }
    } else {
        context.position = None;
    }


    if context.index > i {
        context.index = 0;
    }

    match context.position {
        None => {
            if let Some(distance) = context.distance.take() {
                camera.distance = distance;
            }

            if let Some(tilt) = context.tilt.take() {
                camera.tilt = tilt;
            }

            if input.is_action_deactivated(Action::RotateCamera) {
                context.control_active = true;
            }

            if !input.is_action_hold(Action::RotateCamera) {
                camera.pan = if context.control_active {
                    let mut pan_error = PI - camera.pan;

                    if pan_error.abs() > PI {
                        pan_error = -pan_error.signum()*PI + pan_error%PI;
                    }

                    if pan_error.abs() < PI/128.0 {
                        context.control_active = false;
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
