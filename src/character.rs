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
            .add_system(update_player_speed)
            .add_system(update_gravity_force)
            .add_system(update_action_force)
            .add_system(update_movement_force)
            .add_system(update_player_speed)
            .add_system_set(ConditionSet::new().with_system(update_player_pos).into());
    }
}

#[derive(Default, Debug)]
struct CharacterForces {
    gravity: Vec3,
    movement: Vec3,
    actions: Vec3,
}

struct CharacterSpeedSettings {
    pub base: f32,
    pub run: f32,
    pub crouch: f32,
    pub slide: f32,
    current: f32,
}

impl CharacterSpeedSettings {
    pub fn current(&self) -> f32 {
        self.current
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
            current: 1.0,
        },
        forces: Default::default(),
    };
    commands
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
        .insert(KinematicCharacterControllerOutput::default());
}

fn update_player_state(
    q: Query<(
        &KinematicCharacterControllerOutput,
        &ActionState<CharacterActions>,
    )>,
    mut commands: Commands,
    state: Res<CurrentState<CharacterState>>,
) {
    let (physics, actions) = q.single();

    let new_state = CharacterState::transition(state.0, physics, actions);

    if let Some(new_state) = new_state {
        commands.insert_resource(NextState(new_state));
    }
}

fn update_gravity_force(
    mut q: Query<(
        &mut CharacterMovementController,
        &KinematicCharacterControllerOutput,
    )>,
    time: Res<Time>,
) {
    let (mut movement, physics) = q.single_mut();

    if physics.grounded {
        movement.forces.gravity = Vec3::ZERO;
    } else {
        movement.forces.gravity += Vec3::new(0.0, -9.81 * 5.0, 0.0) * time.delta_seconds();
    };
}

fn update_movement_force(
    mut q: Query<(
        &mut CharacterMovementController,
        &ActionState<CharacterMovement>,
    )>,
) {
    let (mut character, movement) = q.single_mut();
    let speed = character.speed.current();

    character.forces.movement = movement
        .get_pressed()
        .iter()
        .map(|movement| movement.into_vec())
        .sum::<Vec3>()
        .mul(speed)
        .clamp_length(0., speed);
}

fn update_player_speed(
    mut q: Query<&mut CharacterMovementController>,
    state: Res<CurrentState<CharacterState>>,
) {
    let mut character = q.single_mut();

    character.speed.current = match state.0 {
        CharacterState::Run => character.speed.run,
        CharacterState::Walk => character.speed.base,
        CharacterState::Slide => character.speed.slide,
        CharacterState::Crouch => character.speed.crouch,
        CharacterState::Jump => character.speed.base,
        CharacterState::Fall => character.speed.base,
        CharacterState::Idle => 1.0,
    }
}

fn update_action_force(
    mut q: Query<&mut CharacterMovementController>,
    state: Res<CurrentState<CharacterState>>,
) {
    let mut character = q.single_mut();
    let speed = character.speed.current();

    let action_force = match state.0 {
        CharacterState::Slide => character.forces.movement,
        CharacterState::Jump => Vec3::new(0., 100., 0.),
        _ => Vec3::ZERO,
    };

    action_force.mul(speed).clamp_length(0., speed);

    character.forces.actions = action_force;
}

#[rustfmt::skip]
fn update_player_pos(
    mut q: Query<(
        &mut KinematicCharacterController,
        &CharacterMovementController,
        &Transform
    )>,
    time: Res<Time>,
) {
    let (mut controller,  character, transform) = q.single_mut();

    let gravity = character.forces.gravity;
    let movement = character.forces.movement;
    let actions = character.forces.actions;

    let direction = movement
        .add(actions) 
        .add(gravity)
        .mul(time.delta_seconds());

    controller.translation = Some(transform.rotation * direction);
}
