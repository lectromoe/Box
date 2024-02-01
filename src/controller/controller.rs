use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Mul};

pub struct BoxyControllerPlugin;
impl Plugin for BoxyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControllerSpeed>()
            .add_state::<ControllerState>()
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

#[derive(Component)]
pub struct MovementController {
    speed: ControllerSpeedSettings,
    forces: ControllerForces,
    jump_force: f32,
    grounded: bool,
    height: f32,
    mass: f32,
}

impl MovementController {
    pub fn grounded(&self) -> bool {
        self.grounded
    }
    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded;
    }
    pub fn jump_force(&self) -> f32 {
        self.jump_force
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn mass(&self) -> f32 {
        self.mass
    }
    pub fn speed(&self) -> &ControllerSpeedSettings {
        &self.speed
    }
    pub fn set_speed(&mut self, speed: ControllerSpeedSettings) {
        self.speed = speed;
    }
    pub fn set_jump_force(&mut self, jump_force: f32) {
        self.jump_force = jump_force;
    }
    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }
    pub fn forces(&self) -> &ControllerForces {
        &self.forces
    }
    pub fn set_forces(&mut self, forces: ControllerForces) {
        self.forces = forces;
    }
}

fn spawn_player(mut commands: Commands) {
    let settings = MovementController {
        speed: ControllerSpeedSettings {
            base: ControllerSpeed(10.),
            run: ControllerSpeed(20.),
            crouch: ControllerSpeed(5.),
            slide: ControllerSpeed(25.),
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
        .insert(KinematicCharacterControllerOutput::default())
        .insert(CameraTarget {});
}

fn update_gravity_force(
    mut q: Query<(&mut MovementController, &KinematicCharacterControllerOutput)>,
    time: Res<Time>,
) {
    let (mut character, physics) = q.single_mut();

    let mass = character.mass;
    let gravity_constant = Vec3::new(0.0, -9.81, 0.0);
    let gravity_force = character.forces.gravity();

    if physics.grounded {
        character.forces.set_gravity(Vec3::ZERO);
    } else {
        character
            .forces
            .set_gravity(gravity_force + (gravity_constant * mass * time.delta_seconds()));
    };
}

fn update_movement_force(
    mut q: Query<(&mut MovementController, &ActionState<CharacterMovement>)>,

    speed: ResMut<ControllerSpeed>,
) {
    let (mut character, movement) = q.single_mut();
    let speed = speed;

    let movement = movement
        .get_pressed()
        .iter()
        .map(|movement| movement.into_vec())
        .sum::<Vec3>()
        .mul(speed.get() as f32)
        .clamp_length(0., speed.get());

    character.forces.set_movement(movement);
}

fn update_player_speed(
    q: Query<&MovementController>,
    state: Res<State<ControllerState>>,
    mut speed: ResMut<ControllerSpeed>,
) {
    let character = q.single();

    let new_speed: Option<ControllerSpeed> = match state.get() {
        ControllerState::Run => Some(character.speed.run),
        ControllerState::Walk => Some(character.speed.base),
        ControllerState::Slide => Some(character.speed.slide),
        ControllerState::Crouch => Some(character.speed.crouch),
        _ => None,
    };

    if let Some(new_speed) = new_speed {
        *speed = new_speed
    }
}

fn update_action_force(mut q: Query<&mut MovementController>, state: Res<State<ControllerState>>) {
    let mut character = q.single_mut();
    let move_direction = character.forces.movement();

    let action_force = match state.get() {
        ControllerState::Slide => move_direction,
        ControllerState::Jump => Vec3::new(0., character.jump_force, 0.),
        _ => Vec3::ZERO,
    };

    character.forces.set_actions(action_force);
}

#[rustfmt::skip]
fn update_player_pos(
    mut q: Query<(
        &mut KinematicCharacterController,
        &MovementController,
        &Transform
    )>,
    time: Res<Time>,
) {
    let (mut controller,  character, transform) = q.single_mut();

    let gravity = character.forces.gravity();
    let movement = character.forces.movement();
    let actions = character.forces.actions();

    let direction = movement
        .add(actions) 
        .add(gravity)
        .mul(time.delta_seconds());

    controller.translation = Some(transform.rotation * direction);
}
