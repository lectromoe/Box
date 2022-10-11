use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::fmt::Debug;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug_camera)
            .add_plugin(InputManagerPlugin::<CameraAction>::default())
            .add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_system(debug_camera_actions)
            .add_system(debug_camera_movement);
    }
}

#[derive(Component)]
pub struct DebugCamera {
    pub locked_in: bool,
    pub focus: Vec3,
    pub radius: f32,
    pub sens: f32,
    pub look_sens: f32,
    pub upside_down: bool,
}

impl Default for DebugCamera {
    fn default() -> Self {
        DebugCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            sens: 0.005,
            look_sens: 0.005,
            upside_down: false,
            locked_in: false,
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
    RotateTrigger,
    Pan,
    PanTrigger,
    Zoom,
    ToggleSens,
    LockIn,
    LockOut,
}

fn spawn_debug_camera(mut commands: Commands) {
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
                .insert(MouseButton::Right, CameraAction::RotateTrigger)
                .insert(MouseButton::Middle, CameraAction::PanTrigger)
                .insert(KeyCode::LShift, CameraAction::ToggleSens)
                .insert(KeyCode::Escape, CameraAction::LockOut)
                .insert_chord([KeyCode::LShift, KeyCode::C], CameraAction::LockIn)
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

fn debug_camera_actions(
    mut q: Query<(&mut DebugCamera, &mut Transform, &ActionState<CameraAction>)>,
) {
    let (mut camera, mut transform, actions) = q.single_mut();
    let pan = actions.axis_pair(CameraAction::Pan).unwrap();
    let zoom = actions.axis_pair(CameraAction::Zoom).unwrap();
    let mut rotate_triggered = false;

    actions
        .get_pressed()
        .iter()
        .for_each(|action| match action {
            CameraAction::RotateTrigger => {
                rotate_triggered = true;
            }
            CameraAction::PanTrigger => {
                let dx = transform.rotation * Vec3::X * camera.sens * pan.x();
                let dy = transform.rotation * Vec3::Y * camera.sens * pan.y();
                transform.translation = transform.translation - dx + dy;
            }
            _ => (),
        });

    actions
        .get_just_pressed()
        .iter()
        .for_each(|action| match action {
            CameraAction::LockIn => {
                camera.locked_in = true;
            }
            CameraAction::LockOut => {
                camera.locked_in = false;
            }
            CameraAction::ToggleSens => {
                camera.sens *= 5.0;
            }
            _ => (),
        });

    actions
        .get_just_released()
        .iter()
        .for_each(|action| if let CameraAction::ToggleSens = action {
            camera.sens *= 0.2;
        });

    if camera.locked_in || rotate_triggered {
        transform.rotation =
            Quat::from_rotation_y(-pan.x() * camera.look_sens) * transform.rotation;
        transform.rotation *= Quat::from_rotation_x(-pan.y() * camera.look_sens);
    }

    // TODO: Handle zoom
    // if zoom.length_squared() == 0.0 {
    //     transform.rotation = Quat::from_rotation_y(-zoom.x() * camera.sens) * transform.rotation;
    //     transform.rotation *= Quat::from_rotation_x(-zoom.y() * camera.sens);
    // }
}

fn debug_camera_movement(
    mut q: Query<(&mut Transform, &DebugCamera, &ActionState<CameraMovement>)>,
) {
    let (mut transform, camera, movement) = q.single_mut();

    movement
        .get_pressed()
        .iter()
        .for_each(|movement| move_action(&mut transform, camera, movement));
}

fn move_action(transform: &mut Mut<Transform>, camera: &DebugCamera, movement: &CameraMovement) {
    let mut direction = movement.into_vec();

    // Apply up and down movements on global axis
    if *movement != CameraMovement::Up && *movement != CameraMovement::Down {
        direction = transform.rotation * direction;
    }

    transform.translation += direction * camera.sens;
}
