use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use CharacterState::*;

#[derive(Resource, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterState {
    Run,
    Idle,
    Walk,
    Slide,
    Crouch,
    Jump,
    Fall,
}

impl Default for CharacterState {
    fn default() -> Self {
        return Self::Walk;
    }
}

#[derive(Default, Resource, Debug, Clone, Copy, PartialEq, Deref, DerefMut)]
pub struct CharacterSpeed(pub f32);

impl CharacterSpeed {
    pub(crate) fn get(&self) -> f32 {
        self.0
    }
}

#[rustfmt::skip]
pub fn update_player_state(
    mut q: Query<(
        &mut CharacterMovementController,
        &KinematicCharacterControllerOutput,
        &ActionState<CharacterActions>,
    )>,
    state: ResMut<State<CharacterState>>,
    mut next_state: ResMut<NextState<CharacterState>>,
) {
    let (mut character, physics, actions) = q.single_mut();
    let mut new_state = None;
    let grounded = character.grounded();

    match state.get() {
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
        next_state.set(new_state)
    }
}
