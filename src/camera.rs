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
    Pan,
    Tilt,
    Zoom,
    Orbit,
    MoveTrigger,
    OrbitTrigger,
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
                .insert(MouseButton::Right, CameraBinds::OrbitTrigger)
                .insert(MouseButton::Middle, CameraBinds::MoveTrigger)
                .build(),
            action_state: ActionState::default(),
        });
}

fn update_debug_camera(
    windows: Res<Windows>,
    mut q: Query<(&mut Transform, &ActionState<CameraBinds>), With<DebugCamera>>,
) {
    let (mut transform, action_state) = q.single_mut();
    let mut pan = action_state.action_data(CameraBinds::Pan).value;
    let mut tilt = action_state.action_data(CameraBinds::Tilt).value;
    let mut zoom = action_state.axis_pair(CameraBinds::Zoom).unwrap();

    if action_state.pressed(CameraBinds::MoveTrigger) {
        transform.translation.x -= 0.005 * pan;
        transform.translation.y -= 0.005 * tilt;
    }

}
