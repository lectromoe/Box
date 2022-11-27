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
pub fn update_player_state(
    mut q: Query<(
        &mut CharacterMovementController,
        &KinematicCharacterControllerOutput,
        &ActionState<CharacterActions>,
    )>,
    mut state: ResMut<State<CharacterState>>,
) {
    let (mut character, physics, actions) = q.single_mut();
    let mut new_state = None;
    let grounded = character.grounded();

    match state.current() {
        Run => {
            if actions.just_released(CharacterActions::Sprint) { new_state = Some(Walk) }
            if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Slide) }
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if physics.effective_translation == Vec3::ZERO { new_state = Some(Idle) }
            if !grounded { new_state = Some(Fall) }
        }
        Walk => {
            if actions.pressed(CharacterActions::Sprint) { new_state = Some(Run) }
            if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Crouch) }
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if physics.effective_translation == Vec3::ZERO { new_state = Some(Idle) }
            if !grounded { new_state = Some(Fall) }
        }
        Slide => {
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if actions.just_released(CharacterActions::Crouch) { new_state = Some(Run) }
            if !grounded { new_state = Some(Fall) }
        }
        Jump => {
            if physics.effective_translation.y < 0.0 { new_state = Some(Fall) }
        }
        Idle => {
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Crouch) }
            if physics.effective_translation != Vec3::ZERO { new_state = Some(Walk) }
            if !grounded { new_state = Some(Fall) }
        }
        Crouch => {
            if actions.just_released(CharacterActions::Crouch) { 
                new_state = Some(Idle);
                if physics.effective_translation != Vec3::ZERO { 
                    new_state = Some(Walk);
                    if actions.pressed(CharacterActions::Sprint) { 
                        new_state = Some(Run) 
                    } 
                } 
            }
            if !grounded { new_state = Some(Fall) }
        }
        Fall => {
            if grounded { 
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

    character.set_grounded(physics.grounded);

    if let Some(new_state) = new_state {
        state.set(new_state).unwrap_or_default();
    }
}
