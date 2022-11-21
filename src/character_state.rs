use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

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

impl CharacterState {
    pub fn transition(
        state: CharacterState,
        physics: &KinematicCharacterControllerOutput,
        actions: &ActionState<CharacterActions>,
    ) -> Option<CharacterState> {
        use CharacterState::*;

        let mut next_state = None;

        match state {
            Run => {
                if actions.just_released(CharacterActions::Sprint) {
                    next_state = Some(Walk)
                }

                if actions.just_pressed(CharacterActions::Crouch) {
                    next_state = Some(Slide)
                }

                if actions.just_pressed(CharacterActions::Jump) {
                    next_state = Some(Jump)
                }
            }
            Walk => {
                if actions.pressed(CharacterActions::Sprint) {
                    next_state = Some(Run)
                }

                if actions.just_pressed(CharacterActions::Crouch) {
                    next_state = Some(Crouch)
                }

                if actions.just_pressed(CharacterActions::Jump) {
                    next_state = Some(Jump)
                }
            }
            Slide => {
                if actions.just_pressed(CharacterActions::Jump) {
                    next_state = Some(Jump)
                }

                if actions.just_released(CharacterActions::Crouch) {
                    next_state = Some(Run)
                }
            }
            Crouch => {
                if actions.just_released(CharacterActions::Crouch) {
                    next_state = Some(Idle)
                }
            }
            Jump => {
                if physics.grounded {
                    next_state = Some(Idle)
                }
            }
            Fall => {
                if physics.grounded {
                    next_state = Some(Idle)
                }
            }
            Idle => {
                if physics.effective_translation != Vec3::ZERO {
                    next_state = Some(Walk)
                }
            }
        }

        if physics.effective_translation == Vec3::ZERO && state != Idle {
            next_state = Some(Idle)
        }

        if state != Jump && !physics.grounded && state != Fall {
            next_state = Some(Fall)
        }

        next_state
    }
}
