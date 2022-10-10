use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::fmt::Debug;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug_camera)
            .add_plugin(InputManagerPlugin::<CameraBinds>::default())
            .add_system(update_debug_camera);
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backwards,
    Up,
    Down,
    None,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}
impl Direction {
    pub fn into_vec(self) -> Vec3 {
        match self {
            Direction::Up => Vec3::Y,
            Direction::Down => Vec3::NEG_Y,
            Direction::Right => Vec3::X,
            Direction::Left => Vec3::NEG_X,
            Direction::Forward => Vec3::Z,
            Direction::Backwards => Vec3::NEG_Z,
            Direction::None => Vec3::ZERO,
        }
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraBinds {
    Arc,
    ArcTrigger,
    Pan,
    Tilt,
    PanTrigger,
    Zoom,
    Move(Direction),
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
                .insert(MouseButton::Middle, CameraBinds::PanTrigger)
                .insert(KeyCode::W, CameraBinds::Move(Direction::Forward))
                .insert(KeyCode::A, CameraBinds::Move(Direction::Left))
                .insert(KeyCode::S, CameraBinds::Move(Direction::Right))
                .insert(KeyCode::D, CameraBinds::Move(Direction::Backwards))
                .insert(KeyCode::Space, CameraBinds::Move(Direction::Up))
                .insert(KeyCode::LShift, CameraBinds::Move(Direction::Down))
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

    action_state
        .get_pressed()
        .iter()
        .for_each(|action| match action {
            CameraBinds::ArcTrigger => arc_movement(&mut transform, pan, tilt),
            CameraBinds::PanTrigger => pan_movement(&mut transform, pan, tilt),
            CameraBinds::Move(direction) => direction_movement(&mut transform, direction),
            _ => (),
        });
}

fn arc_movement(transform: &mut Mut<Transform>, pan: f32, tilt: f32) {
    transform.rotation = Quat::from_rotation_y(-pan * 0.005) * transform.rotation;
    transform.rotation *= Quat::from_rotation_x(-tilt * 0.005);
}

fn pan_movement(transform: &mut Mut<Transform>, pan: f32, tilt: f32) {
    let dx = transform.rotation * Vec3::X * 0.005 * pan;
    let dy = transform.rotation * Vec3::Y * 0.005 * tilt;
    transform.translation -= dx;
    transform.translation += dy;
}

fn direction_movement(transform: &mut Mut<Transform>, direction: &Direction) {
    bevy::log::info!("{:?}", direction);
    bevy::log::info!("{}", direction.into_vec());
    transform.translation += direction.into_vec();
}
