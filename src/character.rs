use bevy::prelude::*;
use bevy_rapier3d::{prelude::*};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Mul};

#[derive(Component)]
struct CharacterSettings {
    speed: f32,
    run_speed: f32,
    crouch_speed: f32,
    slide_speed: f32,
    height: f32,
}

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(CharacterState::Walk)
            .add_plugin(InputManagerPlugin::<CharacterMovement>::default())
            .add_plugin(InputManagerPlugin::<CharacterActions>::default())
            .add_startup_system(spawn_player)
            .add_system(update_player_state)
            .add_system_set(
                ConditionSet::new()
                    // .run_in_state(CharacterState::Walk)
                    .with_system(update_player_pos)
                    .into(),
            );
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterState {
    Run,
    Idle,
    Walk,
    Slide,
    Crouch,
    Jump,
    Fall,
}

impl CharacterState {
    fn transition(
        from: CharacterState,
        controller: &KinematicCharacterControllerOutput,
        actions: &ActionState<CharacterActions>,
    ) -> Option<CharacterState> {
        use CharacterState::*;

        let mut next_state = None;
        match from {
            Run => {
                if actions.just_released(CharacterActions::Sprint) {
                    next_state = Some(Walk)
                }

                if controller.effective_translation == Vec3::ZERO {
                    next_state = Some(Idle)
                }
            }
            Walk => {
                if actions.just_pressed(CharacterActions::Sprint) {
                    next_state = Some(Run)
                }

                if controller.effective_translation == Vec3::ZERO {
                    next_state = Some(Idle)
                }
            }
            Slide => {
                if actions.just_pressed(CharacterActions::Jump) {
                    next_state = Some(Jump)
                }

                if actions.just_released(CharacterActions::Crouch) {
                    next_state = Some(Run)
                }
            }
            Crouch => {
                if actions.just_released(CharacterActions::Crouch) {
                    next_state = Some(Idle)
                }
            }
            Jump => {
                if controller.grounded {
                    next_state = Some(Idle)
                }
            }
            Fall => {
                if controller.grounded {
                    next_state = Some(Idle)
                }
            }
            Idle => {
                if controller.effective_translation != Vec3::ZERO {
                    next_state = Some(Walk)
                }
            }
        }

        if from != Jump && !controller.grounded {
            next_state = Some(Fall)
        }

        next_state
    }
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
    let settings = CharacterSettings {
        height: 2.0,
        speed: 10.0,
        run_speed: 20.0,
        slide_speed: 25.0,
        crouch_speed: 5.0,
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
                // .insert(KeyCode::Space, CharacterMovement::Jump)
                // .insert(KeyCode::LControl, CharacterMovement::Crouch)
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
        });
}

fn update_player_state(
    mut q: Query<(
        &KinematicCharacterControllerOutput,
        &CharacterSettings,
        &ActionState<CharacterMovement>,
        &ActionState<CharacterActions>,
    )>,
    mut commands: Commands,
    state: Res<CurrentState<CharacterState>>,
) {
    for (controller, settings, movement, actions) in q.iter() {
        let new_state = CharacterState::transition(state.0, controller, actions);

        if let Some(state) = new_state {
            commands.insert_resource(NextState(state));
        }
    }
}

fn update_player_pos(
    mut q: Query<(
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
        &CharacterSettings,
        &ActionState<CharacterMovement>,
    )>,
    state: Res<CurrentState<CharacterState>>,
    time: Res<Time>,
) {
    let (mut controller, physics, settings, movement) = q.single_mut();

    let speed = match state.0 {
        CharacterState::Idle => settings.speed,
        CharacterState::Walk => settings.speed,
        CharacterState::Run => settings.run_speed,
        CharacterState::Crouch => settings.crouch_speed,
        _ => settings.speed,
    };

    let mut gravity = 0.0;
    if let Some(physics) = physics {
        gravity = if physics.grounded { 0.1 } else { 0.0 };
    };

    let direction = movement
        .get_pressed()
        .iter()
        .map(|movement| movement.into_vec())
        .sum::<Vec3>()
        .mul(speed * 10.)
        .add(Vec3::new(0.0, -gravity, 0.0))
        .clamp_length(0., speed)
        .mul(time.delta_seconds());

    controller.translation = Some(direction);
}
