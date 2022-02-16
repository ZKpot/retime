use std::f32::consts::PI;

use dotrix::{
    Transform, World, Input,
    ecs::{ Mut, Const, Context, },
    math::{ Vec3, },
};

use crate::player;
use crate::actions::Action;

const CAMERA_SPD: f32 = 0.05; // < 1

pub struct Camera {
    control_active: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            control_active: false,
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

        if input.is_action_deactivated(Action::RotateCamera) {
            context.control_active = true;
        }

        if !input.is_action_hold(Action::RotateCamera) {
            camera.pan = if context.control_active {
                let mut y_angle_error = PI - camera.pan;

                if y_angle_error.abs() > PI {
                    y_angle_error = -y_angle_error.signum()*PI + y_angle_error%PI;
                }

                if y_angle_error.abs() < PI/128.0 {
                    context.control_active = false;
                    PI
                } else {
                    camera.pan + y_angle_error * CAMERA_SPD
                }
            } else {
                PI
            }
        }
    }
}
