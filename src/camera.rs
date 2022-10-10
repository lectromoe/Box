use bevy::prelude::*;
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};
use std::fmt::Debug;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug_camera)
            .add_plugin(InputManagerPlugin::<CameraAction>::default())
            .add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_system(update_debug_camera);
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
    Arc,
    ArcTrigger,
    Pan,
    PanTrigger,
    Zoom,
    ToggleSens,
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
        .insert_bundle(InputManagerBundle::<CameraAction> {
            input_map: InputMap::default()
                .insert(DualAxis::mouse_motion(), CameraAction::Pan)
                .insert(DualAxis::mouse_wheel(), CameraAction::Zoom)
                .insert(MouseButton::Right, CameraAction::ArcTrigger)
                .insert(MouseButton::Middle, CameraAction::PanTrigger)
                .insert(KeyCode::LShift, CameraAction::ToggleSens)
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

fn update_debug_camera(
    windows: Res<Windows>,
    mut q: Query<(
        &mut DebugCamera,
        &mut Transform,
        &ActionState<CameraAction>,
        &ActionState<CameraMovement>,
    )>,
) {
    let (mut camera, mut transform, actions, movement) = q.single_mut();

    let pan = actions.axis_pair(CameraAction::Pan).unwrap();
    let zoom = actions.axis_pair(CameraAction::Zoom).unwrap();

    camera.upside_down = (transform.rotation * Vec3::Y).y <= 0.0;

    actions
        .get_pressed()
        .iter()
        .for_each(|action| match action {
            CameraAction::ArcTrigger => arc_movement(&mut transform, pan),
            CameraAction::PanTrigger => pan_movement(&mut transform, pan),
            // TODO: Zoom
            _ => (),
        });

    movement
        .get_pressed()
        .iter()
        .for_each(|movement| move_action(&mut transform, movement));

    if zoom.length_squared() == 0.0 {
        zoom_action(&mut transform, zoom)
    }
}

fn arc_movement(transform: &mut Mut<Transform>, pan: DualAxisData) {
    transform.rotation = Quat::from_rotation_y(-pan.x() * 0.005) * transform.rotation;
    transform.rotation *= Quat::from_rotation_x(-pan.y() * 0.005);
}

fn pan_movement(transform: &mut Mut<Transform>, pan: DualAxisData) {
    let dx = transform.rotation * Vec3::X * 0.005 * pan.x();
    let dy = transform.rotation * Vec3::Y * 0.005 * pan.y();
    transform.translation = transform.translation - dx + dy;
}

fn zoom_action(transform: &mut Mut<Transform>, zoom: DualAxisData) {
    transform.rotation = Quat::from_rotation_y(-zoom.x() * 0.005) * transform.rotation;
    transform.rotation *= Quat::from_rotation_x(-zoom.y() * 0.005);
}

fn move_action(transform: &mut Mut<Transform>, movement: &CameraMovement) {
    let mut direction = match movement {
        CameraMovement::Up => Vec3::Y,
        CameraMovement::Down => Vec3::NEG_Y,
        CameraMovement::Right => Vec3::X,
        CameraMovement::Left => Vec3::NEG_X,
        CameraMovement::Backwards => Vec3::Z,
        CameraMovement::Forward => Vec3::NEG_Z,
    };

    // Apply up and down movements on global axis
    if *movement != CameraMovement::Up && *movement != CameraMovement::Down {
        direction = transform.rotation * direction;
    }

    transform.translation += direction * 0.005;
}
