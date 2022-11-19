use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::{Add, Mul};

#[derive(Component)]
struct CharacterSettings {
    walk_speed: f32,
    run_speed: f32,
    height: f32,
}

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(CharacterState::Walk)
            .add_plugin(InputManagerPlugin::<CharacterMovement>::default())
            .add_startup_system(spawn_player)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(CharacterState::Walk)
                    .with_system(update_player_pos)
                    .into(),
            );
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterState {
    Run,
    Walk,
    Slide,
    Crouch,
    Airborn,
    Flying,
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
    Crouch,
}

fn spawn_player(mut commands: Commands) {
    let settings = CharacterSettings {
        walk_speed: 5.0,
        run_speed: 10.0,
        height: 2.0,
    };
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.5),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: false,
            }),
            max_slope_climb_angle: 45.0_f32.to_radians(),
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
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
        });
}

fn update_player_pos(
    mut q: Query<(
        &mut KinematicCharacterController,
        &CharacterSettings,
        &ActionState<CharacterMovement>,
    )>,
    state: Res<CurrentState<CharacterState>>,
    time: Res<Time>,
) {
    let (mut controller, settings, movement) = q.single_mut();

    let speed = match state.0 {
        CharacterState::Run => settings.run_speed,
        CharacterState::Walk => settings.walk_speed,
        _ => settings.walk_speed,
    };

    let gravity = Vec3::new(0., -1., 0.);

    let direction = movement
        .get_pressed()
        .iter()
        .map(|movement| movement.into_vec())
        .sum::<Vec3>()
        .mul(speed)
        .add(gravity)
        .mul(time.delta_seconds());

    controller.translation = Some(direction);
}
