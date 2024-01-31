use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, Reflect, Clone, Hash, Debug, Copy, PartialEq, Eq)]
pub enum CameraAction {
    Rotate,
    MoveTrigger,
    Pan,
    PanTrigger,
    Zoom,
    SpeedTrigger,
    ModeCycleTrigger,
}