pub use rapier3d::prelude::*;

use dotrix::{
    World, Transform,
    ecs::{ Mut, Context, Const, },
    math::{ Quat, },
};

#[derive(Clone)]
pub struct PhysicsState {
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub joint_set: JointSet,
    pub ccd_solver: CCDSolver,
    pub integration_parameters: IntegrationParameters,
    pub gravity: Vector<f32>,
}

impl Default for PhysicsState {
    fn default() -> Self {
        Self {
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            integration_parameters: IntegrationParameters::default(),
            gravity: vector![0.0, -9.81, 0.0],
        }
    }
}

pub struct State {
    pub physics: Option<PhysicsState>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            physics: Some(PhysicsState::default()),
        }
    }
}

pub struct Properties {
    pipeline: PhysicsPipeline,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
        }
    }
}

pub fn step(
    mut props: Context<Properties>,
    mut state: Mut<State>,
) {
    let PhysicsState {
        mut island_manager,
        mut broad_phase,
        mut narrow_phase,
        mut rigid_body_set,
        mut collider_set,
        mut joint_set,
        mut ccd_solver,
        integration_parameters,
        gravity,
      } = state.physics.take().expect("physics::State must be defined");

    let physics_hooks = ();
    let event_handler = ();

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

    state.physics = Some(PhysicsState {
        island_manager,
        broad_phase,
        narrow_phase,
        rigid_body_set,
        collider_set,
        joint_set,
        ccd_solver,
        integration_parameters,
        gravity,
    });
}

pub fn update_models(
    world: Const<World>,
    state: Const<State>,
) {

    let query = world.query::<(
        &mut Transform, &RigidBodyHandle
    )>();

    for (transform, rigid_body) in query {
        let rigid_body_set = &state.physics.as_ref()
            .expect("physics::State must be defined")
            .rigid_body_set;

        let body = rigid_body_set.get(*rigid_body).unwrap();

        let position = body.position().translation;
        let rotation = body.position().rotation;

        // update model transfrom
        transform.translate.x = position.x;
        transform.translate.y = position.y;
        transform.translate.z = position.z;

        transform.rotate = Quat::new(
            rotation.into_inner().w,
            rotation.into_inner().i,
            rotation.into_inner().j,
            rotation.into_inner().k,
        );
    }
}
