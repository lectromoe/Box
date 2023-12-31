use crate::prelude::*;
use bevy::{prelude::*, reflect::TypePath};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Deref, Mul};

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterSpeed>()
            .add_plugins(InputManagerPlugin::<CharacterMovement>::default())
            .add_plugins(InputManagerPlugin::<CharacterActions>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    update_player_speed,
                    update_gravity_force,
                    update_action_force,
                    update_movement_force,
                ),
            )
            .add_systems(Update, (update_player_pos, update_player_state));
    }
}

#[derive(Default, Debug)]
struct CharacterForces {
    gravity: Vec3,
    movement: Vec3,
    actions: Vec3,
}

struct CharacterSpeedSettings {
    pub base: CharacterSpeed,
    pub run: CharacterSpeed,
    pub crouch: CharacterSpeed,
    pub slide: CharacterSpeed,
}

#[derive(Component)]
pub struct CharacterMovementController {
    speed: CharacterSpeedSettings,
    forces: CharacterForces,
    jump_force: f32,
    grounded: bool,
    height: f32,
    mass: f32,
}

impl CharacterMovementController {
    pub fn grounded(&self) -> bool {
        self.grounded
    }

    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded;
    }
}

#[derive(Actionlike, Reflect, Clone, Hash, Debug, Copy, PartialEq, Eq)]
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

#[derive(Actionlike, Reflect, Clone, Hash, Debug, Copy, PartialEq, Eq)]
pub enum CharacterActions {
    Jump,
    Sprint,
    Crouch,
}

fn spawn_player(mut commands: Commands) {
    let settings = CharacterMovementController {
        speed: CharacterSpeedSettings {
            base: CharacterSpeed(10.),
            run: CharacterSpeed(20.),
            crouch: CharacterSpeed(5.),
            slide: CharacterSpeed(25.),
        },
        forces: Default::default(),
        jump_force: 30.0,
        height: 2.0,
        mass: 30.0,
        grounded: false,
    };
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.05),
            slide: false,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.5),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: false,
            }),
            max_slope_climb_angle: 45.0_f32.to_radians(),
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: Some(CharacterLength::Absolute(0.1)),
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
                .insert(KeyCode::ControlLeft, CharacterActions::Crouch)
                .insert(KeyCode::ShiftLeft, CharacterActions::Sprint)
                .build(),
            action_state: ActionState::default(),
        })
        .insert(KinematicCharacterControllerOutput::default());
}

fn update_gravity_force(
    mut q: Query<(
        &mut CharacterMovementController,
        &KinematicCharacterControllerOutput,
    )>,
    time: Res<Time>,
) {
    let (mut character, physics) = q.single_mut();

    let mass = character.mass;
    let gravity = Vec3::new(0.0, -9.81, 0.0);

    if physics.grounded {
        character.forces.gravity = Vec3::ZERO;
    } else {
        character.forces.gravity += gravity * mass * time.delta_seconds();
    };
}

fn update_movement_force(
    mut q: Query<(
        &mut CharacterMovementController,
        &ActionState<CharacterMovement>,
    )>,

    speed: ResMut<CharacterSpeed>,
) {
    let (mut character, movement) = q.single_mut();
    let speed = speed;

    character.forces.movement = movement
        .get_pressed()
        .iter()
        .map(|movement| movement.into_vec())
        .sum::<Vec3>()
        .mul(speed.get() as f32)
        .clamp_length(0., speed.get());
}

fn update_player_speed(
    q: Query<&CharacterMovementController>,
    state: Res<State<CharacterState>>,
    mut speed: ResMut<CharacterSpeed>,
) {
    let character = q.single();

    let new_speed: Option<CharacterSpeed> = match state.get() {
        CharacterState::Run => Some(character.speed.run),
        CharacterState::Walk => Some(character.speed.base),
        CharacterState::Slide => Some(character.speed.slide),
        CharacterState::Crouch => Some(character.speed.crouch),
        _ => None,
    };

    if let Some(new_speed) = new_speed {
        *speed = new_speed
    }
}

fn update_action_force(
    mut q: Query<&mut CharacterMovementController>,
    state: Res<State<CharacterState>>,
) {
    let mut character = q.single_mut();
    let move_direction = character.forces.movement;

    let action_force = match state.get() {
        CharacterState::Slide => move_direction,
        CharacterState::Jump => Vec3::new(0., character.jump_force, 0.),
        _ => Vec3::ZERO,
    };

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
