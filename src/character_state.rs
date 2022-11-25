use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use CharacterState::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterState {
    Run,
    Idle,
    Walk,
    Slide,
    Crouch,
    Jump,
    Fall,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct CharacterSpeed(pub i32);

#[rustfmt::skip]
impl CharacterState {
    pub fn transition(
        state: CharacterState,
        physics: &KinematicCharacterControllerOutput,
        actions: &ActionState<CharacterActions>,
    ) -> Option<CharacterState> {
        let mut new_state = None;

        println!("{state:?}, {:?}", physics.effective_translation);
        match state {
            Run => {
                if actions.just_released(CharacterActions::Sprint) { new_state = Some(Walk) }
                if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Slide) }
                if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            }
            Walk => {
                if actions.pressed(CharacterActions::Sprint) { new_state = Some(Run) }
                if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Crouch) }
                if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            }
            Slide => {
                if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
                if actions.just_released(CharacterActions::Crouch) { new_state = Some(Run) }
            }
            Crouch => {
                if actions.just_released(CharacterActions::Crouch) { new_state = Some(Idle) }
            }
            Jump => {
                if physics.effective_translation.y < 0.0 { new_state = Some(Fall) }
            }
            Idle => {
                if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
                if physics.effective_translation != Vec3::ZERO { new_state = Some(Walk) }
            }
            Fall => {
                if physics.grounded { 
                    new_state = Some(Idle);
                    if physics.effective_translation != Vec3::ZERO { 
                        new_state = Some(Walk);
                        if actions.pressed(CharacterActions::Sprint) { 
                            new_state = Some(Run) 
                        } 
                    } 
                }
            }
        }

        if state != Jump && !physics.grounded && state != Fall { new_state = Some(Fall) }

        new_state
    }
}
