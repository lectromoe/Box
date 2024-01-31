use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::camera::*;

#[derive(Component)]
pub struct Camera {
    focus: Vec3,
    radius: f32,
    move_sens: f32,
    look_sens: f32,
    zoom_sens: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            focus: Vec3::ONE,
            radius: 15.0,
            move_sens: 0.005,
            look_sens: 0.005,
            zoom_sens: 0.1,
        }
    }
}

impl Camera {
    pub fn new(focus: Vec3, radius: f32) -> Self {
        Self {
            focus,
            radius,
            ..Default::default()
        }
    }
    pub fn focus(&self) -> Vec3 {
        self.focus
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn move_sens(&self) -> f32 {
        self.move_sens
    }
    pub fn look_sens(&self) -> f32 {
        self.look_sens
    }
    pub fn zoom_sens(&self) -> f32 {
        self.zoom_sens
    }
    pub fn set_focus(&mut self, focus: Vec3) {
        self.focus = focus;
    }
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }
    pub fn set_move_sens(&mut self, move_sens: f32) {
        self.move_sens = move_sens;
    }
    pub fn set_look_sens(&mut self, look_sens: f32) {
        self.look_sens = look_sens;
    }
    pub fn set_zoom_sens(&mut self, zoom_sens: f32) {
        self.zoom_sens = zoom_sens;
    }
}


pub struct BoxyCameraPlugin;
impl Plugin for BoxyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_state::<CameraState>()
            .add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_plugins(InputManagerPlugin::<CameraMovement>::default())
            .add_systems(Update, update_camera_state)
            .add_systems(
                Update,
                update_camera_pos.run_if(in_state(CameraState::Locked)),
            )
            .add_systems(
                Update,
                update_camera_rot.run_if(in_state(CameraState::FirstPerson)),
            )
            .add_systems(
                Update,
                update_camera_orb.run_if(in_state(CameraState::ThirdPerson)),
            )
            .add_systems(
                Update,
                (update_camera_rot, update_camera_pos, update_camera_pan)
                    .run_if(in_state(CameraState::Editor)),
            )
            .add_systems(
                Update,
                (
                    update_camera_rot,
                    update_camera_pos,
                    update_camera_pan,
                    update_camera_zoom,
                )
                    .run_if(in_state(CameraState::FreeFloat)),
            );
    }
}


fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Camera {
            radius,
            ..Default::default()
        })
        .insert(InputManagerBundle::<CameraAction> {
            input_map: InputMap::default()
                .insert(DualAxis::mouse_motion(), CameraAction::Pan)
                .insert(DualAxis::mouse_wheel(), CameraAction::Zoom)
                .insert(MouseButton::Right, CameraAction::MoveTrigger)
                .insert(MouseButton::Middle, CameraAction::PanTrigger)
                .insert(KeyCode::ShiftLeft, CameraAction::SpeedTrigger)
                .insert(KeyCode::C, CameraAction::ModeCycleTrigger)
                .build(),
            action_state: ActionState::default(),
        })
        .insert(InputManagerBundle::<CameraMovement> {
            input_map: InputMap::default()
                .insert(KeyCode::W, CameraMovement::Forward)
                .insert(KeyCode::A, CameraMovement::Left)
                .insert(KeyCode::S, CameraMovement::Back)
                .insert(KeyCode::D, CameraMovement::Right)
                .insert(KeyCode::Space, CameraMovement::Up)
                .insert(KeyCode::ControlLeft, CameraMovement::Down)
                .build(),
            action_state: ActionState::default(),
        });
}

fn update_camera_state(
    mut q: Query<(&mut Camera, &ActionState<CameraAction>)>,
    state: ResMut<State<CameraState>>,
    mut next_state: ResMut<NextState<CameraState>>,
) {
    let (mut camera, actions) = q.single_mut();

    if actions.just_pressed(CameraAction::SpeedTrigger) {
        camera.move_sens *= 5.0;
    };

    if actions.just_released(CameraAction::SpeedTrigger) {
        camera.move_sens *= 0.2;
    };

    if actions.just_pressed(CameraAction::ModeCycleTrigger) {
        next_state.set(state.next());
    };
}

fn update_camera_pan(mut q: Query<(&mut Transform, &Camera, &ActionState<CameraAction>)>) {
    let (mut transform, camera, actions) = q.single_mut();
    let pan = actions.axis_pair(CameraAction::Pan).unwrap();

    if actions.pressed(CameraAction::PanTrigger) {
        let dx = transform.rotation * Vec3::X * camera.move_sens * pan.x();
        let dy = transform.rotation * Vec3::Y * camera.move_sens * pan.y();
        transform.translation = transform.translation - dx + dy;
    }
}

fn update_camera_zoom(mut q: Query<(&mut Projection, &Camera, &ActionState<CameraAction>)>) {
    let (mut projection, camera, actions) = q.single_mut();
    let zoom = actions.axis_pair(CameraAction::Zoom).unwrap();
    if zoom.length_squared() == 0.0 {
        return;
    }

    if let Projection::Perspective(projection) = projection.as_mut() {
        projection.fov += -zoom.y() * camera.look_sens;
    }
}

fn update_camera_rot(
    mut q: Query<(&mut Transform, &Camera, &ActionState<CameraAction>)>,
    state: Res<State<CameraState>>,
) {
    let (mut transform, camera, actions) = q.single_mut();
    let motion = actions.axis_pair(CameraAction::Pan).unwrap();
    let triggered = actions.pressed(CameraAction::MoveTrigger);
    let state = *state.get();

    if state == CameraState::FreeFloat || state == CameraState::FirstPerson || triggered {
        transform.rotation =
            Quat::from_rotation_y(-motion.x() * camera.look_sens) * transform.rotation;
        transform.rotation *= Quat::from_rotation_x(-motion.y() * camera.look_sens);
    }
}

fn update_camera_orb(mut q: Query<(&mut Transform, &Camera, &ActionState<CameraAction>)>) {
    let (mut transform, camera, actions) = q.single_mut();
    let motion = actions.axis_pair(CameraAction::Pan).unwrap();
    let radius = camera.radius;
    let position = camera.focus;

    if motion.length_squared() == 0.0 {
        return;
    }

    let sensitivity = 0.01; // Adjust the sensitivity based on your preference
    let delta_yaw = -sensitivity * motion.x();
    let delta_pitch = sensitivity * motion.y();
    let camera_local_x = transform.local_x();

    transform.translate_around(position, Quat::from_axis_angle(Vec3::Y, delta_yaw));
    transform.translate_around(position, Quat::from_axis_angle(camera_local_x, delta_pitch));
    transform.look_at(position, Vec3::Y);
}

fn update_camera_pos(
    mut q: Query<(
        &mut Transform,
        &Camera,
        &ActionState<CameraMovement>,
        &ActionState<CameraAction>,
    )>,
    state: Res<State<CameraState>>,
) {
    let (mut transform, camera, movement, actions) = q.single_mut();
    let triggered = actions.pressed(CameraAction::MoveTrigger);

    if (*state.get() == CameraState::FreeFloat) || triggered {
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
