use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Mul};

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(CharacterState::Walk)
            .add_plugin(InputManagerPlugin::<CharacterMovement>::default())
            .add_plugin(InputManagerPlugin::<CharacterActions>::default())
            .add_startup_system(spawn_player)
            .add_system(update_player_state)
            .add_system_set(ConditionSet::new().with_system(update_player_pos).into());
    }
}

#[derive(Default, Debug)]
struct CharacterForces {
    gravity: Vec3,
}

struct CharacterSpeedSettings {
    base: f32,
    run: f32,
    crouch: f32,
    slide: f32,
}

impl CharacterSpeedSettings {
    pub fn from_state(&self, state: CharacterState) -> f32 {
        match state {
            CharacterState::Run => self.run,
            CharacterState::Walk => self.base,
            CharacterState::Slide => self.slide,
            CharacterState::Crouch => self.crouch,
            CharacterState::Jump => self.base,
            CharacterState::Fall => self.base,
            CharacterState::Idle => 0.0,
        }
    }
}

#[derive(Component)]
struct CharacterMovementController {
    speed: CharacterSpeedSettings,
    forces: CharacterForces,
    height: f32,
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CharacterMovement {
    Left,
    Right,
    Back,
    Forward,
}

impl CharacterMovement {
    pub fn into_vec(self) -> Vec3 {
        match self {
            CharacterMovement::Right => Vec3::X,
            CharacterMovement::Left => Vec3::NEG_X,
            CharacterMovement::Back => Vec3::Z,
            CharacterMovement::Forward => Vec3::NEG_Z,
        }
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CharacterActions {
    Jump,
    Sprint,
    Crouch,
}

fn spawn_player(mut commands: Commands) {
    let settings = CharacterMovementController {
        height: 2.0,
        speed: CharacterSpeedSettings {
            base: 10.0,
            run: 20.0,
            crouch: 5.0,
            slide: 25.0,
        },
        forces: Default::default(),
    };
    let player = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            slide: false,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.5),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: false,
            }),
            max_slope_climb_angle: 45.0_f32.to_radians(),
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: Some(CharacterLength::Absolute(0.05)),
            ..Default::default()
        })
        .insert(Collider::capsule_y(settings.height / 2., 1.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(settings)
        .insert(InputManagerBundle::<CharacterMovement> {
            input_map: InputMap::default()
                .insert(KeyCode::W, CharacterMovement::Forward)
                .insert(KeyCode::A, CharacterMovement::Left)
                .insert(KeyCode::S, CharacterMovement::Back)
                .insert(KeyCode::D, CharacterMovement::Right)
                .build(),
            action_state: ActionState::default(),
        })
        .insert(InputManagerBundle::<CharacterActions> {
            input_map: InputMap::default()
                .insert(KeyCode::Space, CharacterActions::Jump)
                .insert(KeyCode::LControl, CharacterActions::Crouch)
                .insert(KeyCode::LShift, CharacterActions::Sprint)
                .build(),
            action_state: ActionState::default(),
        })
        .id();
}

fn update_player_state(
    q: Query<(
        Option<&KinematicCharacterControllerOutput>,
        &ActionState<CharacterActions>,
    )>,
    mut commands: Commands,
    state: Res<CurrentState<CharacterState>>,
) {
    let (controller, actions) = q.single();

    if let Some(physics) = controller {
        let new_state = CharacterState::transition(state.0, physics, actions);

        if let Some(new_state) = new_state {
            commands.insert_resource(NextState(new_state));
        }
    }
}

#[rustfmt::skip]
fn update_player_pos(
    mut q: Query<(
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
        &CharacterMovementController,
        &ActionState<CharacterMovement>,
        &Transform
    )>,
    state: Res<CurrentState<CharacterState>>,
    time: Res<Time>,
) {
    let (mut controller, physics, settings, movement, transform) = q.single_mut();
    let state = state.0;
    let Some(physics) = physics else { 
        controller.translation = Some(Vec3::ZERO);
        return;
    };

    let speed = settings.speed.from_state(state);
    let movement = movement
        .get_pressed()
        .iter()
        .map(|movement| movement.into_vec())
        .sum::<Vec3>()
        .mul(speed)
        .clamp_length(0., speed);

    let actions = match state {
        CharacterState::Slide => Vec3::new(10., 0., 0.),
        CharacterState::Jump  => Vec3::new(0., 500., 0.),
        _                     => Vec3::ZERO,
    };

    let gravity = if physics.grounded { Vec3::ZERO } else { Vec3::new(0.0, -9.81, 0.0) };

    let direction = movement 
        .add(actions)
        .add(gravity)
        .mul(time.delta_seconds());

    controller.translation = Some(transform.rotation * direction);
}
