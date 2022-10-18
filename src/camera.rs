use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use std::fmt::Debug;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_loopless_state(CameraState::FreeFloat)
            .add_plugin(InputManagerPlugin::<CameraAction>::default())
            .add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_system(update_camera_state)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(CameraState::Locked)
                    .with_system(update_camera_pos)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(CameraState::Fps)
                    .with_system(update_camera_rot)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(CameraState::Editor)
                    .with_system(update_camera_rot)
                    .with_system(update_camera_pos)
                    .with_system(update_camera_pan)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(CameraState::FreeFloat)
                    .with_system(update_camera_pos)
                    .with_system(update_camera_rot)
                    .with_system(update_camera_pan)
                    .into(),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraState {
    FreeFloat, // Tranlation, Rotation
    Locked,    // Transltaion only
    Fps,       // Rotation only
    Editor,    // Trigger to move
}

#[derive(Component)]
pub struct DebugCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub move_sens: f32,
    pub look_sens: f32,
    pub upside_down: bool,
}

impl Default for DebugCamera {
    fn default() -> Self {
        DebugCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            move_sens: 0.005,
            look_sens: 0.005,
            upside_down: false,
        }
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraMovement {
    Left,
    Right,
    Forward,
    Backwards,
    Up,
    Down,
}

impl CameraMovement {
    pub fn into_vec(self) -> Vec3 {
        match self {
            CameraMovement::Up => Vec3::Y,
            CameraMovement::Down => Vec3::NEG_Y,
            CameraMovement::Right => Vec3::X,
            CameraMovement::Left => Vec3::NEG_X,
            CameraMovement::Backwards => Vec3::Z,
            CameraMovement::Forward => Vec3::NEG_Z,
        }
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraAction {
    Rotate,
    MoveTrigger,
    Pan,
    PanTrigger,
    Zoom,
    SensTrigger,
    FreeFloatToggle,
}

fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(DebugCamera {
            radius,
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<CameraAction> {
            input_map: InputMap::default()
                .insert(DualAxis::mouse_motion(), CameraAction::Pan)
                .insert(DualAxis::mouse_wheel(), CameraAction::Zoom)
                .insert(MouseButton::Right, CameraAction::MoveTrigger)
                .insert(MouseButton::Middle, CameraAction::PanTrigger)
                .insert(KeyCode::LShift, CameraAction::SensTrigger)
                .insert(KeyCode::C, CameraAction::FreeFloatToggle)
                // .insert_chord([KeyCode::LShift, KeyCode::C], CameraAction::LockIn)
                .build(),
            action_state: ActionState::default(),
        })
        .insert_bundle(InputManagerBundle::<CameraMovement> {
            input_map: InputMap::default()
                .insert(KeyCode::W, CameraMovement::Forward)
                .insert(KeyCode::A, CameraMovement::Left)
                .insert(KeyCode::S, CameraMovement::Backwards)
                .insert(KeyCode::D, CameraMovement::Right)
                .insert(KeyCode::Space, CameraMovement::Up)
                .insert(KeyCode::LControl, CameraMovement::Down)
                .build(),
            action_state: ActionState::default(),
        });
}

fn update_camera_state(
    mut q: Query<(&mut DebugCamera, &ActionState<CameraAction>)>,
    state: Res<CurrentState<CameraState>>,
    mut commands: Commands,
) {
    let (mut camera, actions) = q.single_mut();

    if actions.just_pressed(CameraAction::SensTrigger) {
        camera.move_sens *= 5.0;
    };

    if actions.just_released(CameraAction::SensTrigger) {
        camera.move_sens *= 0.2;
    };

    if actions.just_pressed(CameraAction::FreeFloatToggle) {
        match state.0 {
            CameraState::FreeFloat => commands.insert_resource(NextState(CameraState::Editor)),
            _ => commands.insert_resource(NextState(CameraState::FreeFloat)),
        };
    };
}

fn update_camera_pan(mut q: Query<(&mut Transform, &DebugCamera, &ActionState<CameraAction>)>) {
    let (mut transform, camera, actions) = q.single_mut();
    let pan = actions.axis_pair(CameraAction::Pan).unwrap();

    if actions.pressed(CameraAction::PanTrigger) {
        let dx = transform.rotation * Vec3::X * camera.move_sens * pan.x();
        let dy = transform.rotation * Vec3::Y * camera.move_sens * pan.y();
        transform.translation = transform.translation - dx + dy;
    }
}

fn update_camera_zoom(mut q: Query<(&mut Transform, &DebugCamera, &ActionState<CameraAction>)>) {
    let (mut transform, camera, actions) = q.single_mut();
    let zoom = actions.axis_pair(CameraAction::Zoom).unwrap();

    // TODO
}

fn update_camera_rot(
    mut q: Query<(&mut Transform, &DebugCamera, &ActionState<CameraAction>)>,
    state: Res<CurrentState<CameraState>>,
) {
    let (mut transform, camera, actions) = q.single_mut();
    let motion = actions.axis_pair(CameraAction::Pan).unwrap();
    let triggered = actions.pressed(CameraAction::MoveTrigger);

    if state.0 == CameraState::FreeFloat || triggered {
        transform.rotation =
            Quat::from_rotation_y(-motion.x() * camera.look_sens) * transform.rotation;
        transform.rotation *= Quat::from_rotation_x(-motion.y() * camera.look_sens);
    }
}

fn update_camera_pos(
    mut q: Query<(
        &mut Transform,
        &DebugCamera,
        &ActionState<CameraMovement>,
        &ActionState<CameraAction>,
    )>,
    state: Res<CurrentState<CameraState>>,
) {
    let (mut transform, camera, movement, actions) = q.single_mut();
    let triggered = actions.pressed(CameraAction::MoveTrigger);

    if (state.0 == CameraState::FreeFloat) || triggered {
        movement.get_pressed().iter().for_each(|movement| {
            let mut direction = movement.into_vec();

            // Apply up and down movements on global axis
            if *movement != CameraMovement::Up && *movement != CameraMovement::Down {
                direction = transform.rotation * direction;
            }

            transform.translation += direction * camera.move_sens;
        });
    }
}
