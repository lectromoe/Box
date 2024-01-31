use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Reflect, Clone, Hash, Debug, Copy, PartialEq, Eq)]
pub enum CharacterActions {
    Jump,
    Sprint,
    Crouch,
}