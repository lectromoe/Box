use std::fmt::Debug;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug_camera)
            .add_plugin(InputManagerPlugin::<CameraBinds>::default())
            .add_system(update_debug_camera);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraBinds {
    Arc,
    ArcTrigger,
    Pan,
    Tilt,
    MoveTrigger,
    Zoom,
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackwards,
    MoveUp,
    MoveDown,
}

#[derive(Component)]
pub struct DebugCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for DebugCamera {
    fn default() -> Self {
        DebugCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
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
        .insert_bundle(InputManagerBundle::<CameraBinds> {
            input_map: InputMap::default()
                .insert(DualAxis::mouse_motion().x, CameraBinds::Pan)
                .insert(DualAxis::mouse_motion().y, CameraBinds::Tilt)
                .insert(DualAxis::mouse_wheel(), CameraBinds::Zoom)
                .insert(MouseButton::Right, CameraBinds::ArcTrigger)
                .insert(MouseButton::Middle, CameraBinds::MoveTrigger)
                // .insert(InputButton::, CameraBinds::MoveTrigger)
                .build(),
            action_state: ActionState::default(),
        });
}

fn update_debug_camera(
    windows: Res<Windows>,
    mut q: Query<(&mut DebugCamera, &mut Transform, &ActionState<CameraBinds>)>,
) {
    let (mut camera, mut transform, action_state) = q.single_mut();

    let pan = action_state.action_data(CameraBinds::Pan).value;
    let tilt = action_state.action_data(CameraBinds::Tilt).value;
    let zoom = action_state.axis_pair(CameraBinds::Zoom).unwrap();
    camera.upside_down = (transform.rotation * Vec3::Y).y <= 0.0;

    if action_state.pressed(CameraBinds::MoveTrigger) {
        let dx = transform.rotation * Vec3::X * 0.005 * pan;
        let dy = transform.rotation * Vec3::Y * 0.005 * tilt;
        transform.translation -= dx;
        transform.translation += dy;
    }

    if action_state.pressed(CameraBinds::ArcTrigger) {
        transform.rotation = Quat::from_rotation_y(-pan * 0.005) * transform.rotation;
        transform.rotation *= Quat::from_rotation_x(-tilt * 0.005);
    }
}
