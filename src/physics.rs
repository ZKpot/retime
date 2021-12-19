pub use rapier3d::prelude::*;

use dotrix::{
    ecs::{ Mut, Context, },
};

pub struct Properties {
    pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
        }
    }
}

pub fn step(
    mut props: Context<Properties>,
    mut island_manager: Mut<IslandManager>,
    mut broad_phase: Mut<BroadPhase>,
    mut narrow_phase: Mut<NarrowPhase>,
    mut rigid_body_set: Mut<RigidBodySet>,
    mut collider_set: Mut<ColliderSet>,
    mut joint_set: Mut<JointSet>,
    mut ccd_solver: Mut<CCDSolver>,
) {
    let gravity = props.gravity;
    let mut integration_parameters = props.integration_parameters;

    let physics_hooks = ();
    let event_handler = ();

    integration_parameters.dt = 1.0 / 60.0;

    props.pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
        &mut ccd_solver,
        &physics_hooks,
        &event_handler,
    );
}
