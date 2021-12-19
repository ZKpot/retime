use std::f32::consts::PI;

use dotrix::{
    Camera,
    ecs::{ Mut, },
};

pub fn startup (
    mut camera: Mut<Camera>,
) {
    camera.y_angle = PI;
    camera.xz_angle = PI/8.0;
    camera.distance = 10.0;
}

pub fn control (
    mut camera: Mut<Camera>,
) {
    if camera.xz_angle < 0.0 {
        camera.xz_angle = 0.0;
    };
}
