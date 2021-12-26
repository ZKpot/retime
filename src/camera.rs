use std::f32::consts::PI;

use dotrix::{
    Transform, World,
    ecs::{ Mut, Const, },
    math::Point3,
};

use crate::player;

pub fn startup (
    mut camera: Mut<dotrix::Camera>,
) {
    camera.y_angle = PI;
    camera.xz_angle = PI/8.0;
    camera.distance = 10.0;
}

pub fn control (
    world: Const<World>,
    mut camera: Mut<dotrix::Camera>,
) {
    if camera.xz_angle < 0.0 {
        camera.xz_angle = 0.0;
    };

    // make camera follow the player
    let query = world.query::<(
        &mut Transform, &mut player::Properties,
    )>();

    for (transform, props) in query {
        // update camera properties
        camera.target = Point3::new(
            transform.translate.x,
            transform.translate.y,
            transform.translate.z
        );

        camera.y_angle = PI - props.fwd_angle;
    }
}
