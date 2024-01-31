use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, Clone, Reflect, Hash, Debug, Copy, PartialEq, Eq)]
pub enum CameraMovement {
    Left,
    Right,
    Back,
    Forward,
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
            CameraMovement::Back => Vec3::Z,
            CameraMovement::Forward => Vec3::NEG_Z,
        }
    }
}