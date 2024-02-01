use bevy::prelude::*;

#[derive(Default, Resource, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraState {
    #[default]
    FreeFloat, // Tranlation, Rotation
    Locked,      // Transltaion only
    FirstPerson, // Rotation only
    ThirdPerson, // Rotation around object
    Editor,      // Trigger to move
}

pub trait Cycle {
    fn next(&self) -> Self;
}

impl Cycle for CameraState {
    fn next(&self) -> Self {
        const STATES: [CameraState; 5] = [
            CameraState::FreeFloat,
            CameraState::Locked,
            CameraState::FirstPerson,
            CameraState::ThirdPerson,
            CameraState::Editor,
        ];
        let index = STATES.iter().position(|&state| state == *self).unwrap_or(0);
        let next_index = (index + 1) % STATES.len();

        STATES[next_index]
    }
}
