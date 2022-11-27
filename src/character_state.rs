use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
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
    q: Query<(
        &KinematicCharacterControllerOutput,
        &ActionState<CharacterActions>,
    )>,
    mut commands: Commands,
    state: Res<CurrentState<CharacterState>>,
) {
    let (physics, actions) = q.single();
    let state = state.0;
    let mut new_state = None;

    println!("{:?}, {state:?}, {:?}", physics.grounded, physics.effective_translation);

    match state {
        Run => {
            if actions.just_released(CharacterActions::Sprint) { new_state = Some(Walk) }
            if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Slide) }
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if physics.effective_translation == Vec3::ZERO { new_state = Some(Idle) }
        }
        Walk => {
            if actions.pressed(CharacterActions::Sprint) { new_state = Some(Run) }
            if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Crouch) }
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if physics.effective_translation == Vec3::ZERO { new_state = Some(Idle) }
        }
        Slide => {
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if actions.just_released(CharacterActions::Crouch) { new_state = Some(Run) }
        }
        Jump => {
            if physics.effective_translation.y < 0.0 { new_state = Some(Fall) }
        }
        Idle => {
            if actions.just_pressed(CharacterActions::Jump) { new_state = Some(Jump) }
            if actions.just_pressed(CharacterActions::Crouch) { new_state = Some(Crouch) }
            if physics.effective_translation != Vec3::ZERO { new_state = Some(Walk) }
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

    if let Some(new_state) = new_state {
        commands.insert_resource(NextState(new_state));
    }
}
