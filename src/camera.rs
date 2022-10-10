use bevy::{prelude::*, utils::label::DynEq};
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};
use std::fmt::Debug;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug_camera)
            .add_plugin(InputManagerPlugin::<CameraBind>::default())
            .add_system(update_debug_camera);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraBind {
    Arc,
    ArcTrigger,
    Pan,
    PanTrigger,
    Zoom,
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackwards,
    MoveUp,
    MoveDown,
    ToggleSens,
}
impl CameraBind {
    const MOVE_ACTIONS: [Self; 6] = [
        CameraBind::MoveLeft,
        CameraBind::MoveRight,
        CameraBind::MoveForward,
        CameraBind::MoveBackwards,
        CameraBind::MoveUp,
        CameraBind::MoveDown,
    ];

    pub fn is_move_action(&self) -> bool {
        Self::MOVE_ACTIONS.contains(self)
    }

    pub fn into_vec(self) -> Vec3 {
        match self {
            CameraBind::MoveUp => Vec3::Y,
            CameraBind::MoveDown => Vec3::NEG_Y,
            CameraBind::MoveRight => Vec3::X,
            CameraBind::MoveLeft => Vec3::NEG_X,
            CameraBind::MoveBackwards => Vec3::Z,
            CameraBind::MoveForward => Vec3::NEG_Z,
            _ => Vec3::ZERO,
        }
    }
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
        .insert_bundle(InputManagerBundle::<CameraBind> {
            input_map: InputMap::default()
                .insert(DualAxis::mouse_motion(), CameraBind::Pan)
                .insert(DualAxis::mouse_wheel(), CameraBind::Zoom)
                .insert(MouseButton::Right, CameraBind::ArcTrigger)
                .insert(MouseButton::Middle, CameraBind::PanTrigger)
                .insert(KeyCode::W, CameraBind::MoveForward)
                .insert(KeyCode::A, CameraBind::MoveLeft)
                .insert(KeyCode::S, CameraBind::MoveBackwards)
                .insert(KeyCode::D, CameraBind::MoveRight)
                .insert(KeyCode::Space, CameraBind::MoveUp)
                .insert(KeyCode::LControl, CameraBind::MoveDown)
                .insert(KeyCode::LShift, CameraBind::ToggleSens)
                .build(),
            action_state: ActionState::default(),
        });
}

fn update_debug_camera(
    windows: Res<Windows>,
    mut q: Query<(&mut DebugCamera, &mut Transform, &ActionState<CameraBind>)>,
) {
    let (mut camera, mut transform, action_state) = q.single_mut();

    let pan = action_state.axis_pair(CameraBind::Pan).unwrap();
    let zoom = action_state.axis_pair(CameraBind::Zoom).unwrap();

    camera.upside_down = (transform.rotation * Vec3::Y).y <= 0.0;

    action_state
        .get_pressed()
        .iter()
        .for_each(|action| match action {
            CameraBind::ArcTrigger => arc_movement(&mut transform, pan),
            CameraBind::PanTrigger => pan_movement(&mut transform, pan),
            x if x.is_move_action() => move_action(&mut transform, x),
            _ => (),
        });

    if zoom.length_squared() == 0.0 {
        zoom_action(&mut transform, zoom)
    }
}

fn zoom_action(transform: &mut Mut<Transform>, zoom: DualAxisData) {
    transform.rotation = Quat::from_rotation_y(-zoom.x() * 0.005) * transform.rotation;
    transform.rotation *= Quat::from_rotation_x(-zoom.y() * 0.005);
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

fn move_action(transform: &mut Mut<Transform>, action: &CameraBind) {
    let mut direction = match action {
        CameraBind::MoveUp => Vec3::Y,
        CameraBind::MoveDown => Vec3::NEG_Y,
        CameraBind::MoveRight => Vec3::X,
        CameraBind::MoveLeft => Vec3::NEG_X,
        CameraBind::MoveBackwards => Vec3::Z,
        CameraBind::MoveForward => Vec3::NEG_Z,
        _ => Vec3::ZERO,
    };

    // Apply up and down movements on global axis
    if *action != CameraBind::MoveUp && *action != CameraBind::MoveDown {
        direction = transform.rotation * direction;
    }
    transform.translation += direction * 0.005;
}
