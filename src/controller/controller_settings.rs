use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct ControllerForces {
    gravity: Vec3,
    movement: Vec3,
    actions: Vec3,
}

impl ControllerForces {
    pub fn new() -> Self {
        ControllerForces {
            gravity: Vec3::ZERO,
            movement: Vec3::ZERO,
            actions: Vec3::ZERO,
        }
    }
    pub fn gravity(&self) -> Vec3 {
        self.gravity
    }
    pub fn movement(&self) -> Vec3 {
        self.movement
    }
    pub fn actions(&self) -> Vec3 {
        self.actions
    }
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.gravity = gravity;
    }
    pub fn set_movement(&mut self, movement: Vec3) {
        self.movement = movement;
    }
    pub fn set_actions(&mut self, actions: Vec3) {
        self.actions = actions;
    }
}

pub struct ControllerSpeedSettings {
    pub base: ControllerSpeed,
    pub run: ControllerSpeed,
    pub crouch: ControllerSpeed,
    pub slide: ControllerSpeed,
}

impl Default for ControllerSpeedSettings {
    fn default() -> Self {
        return ControllerSpeedSettings {
            base: ControllerSpeed(10.),
            run: ControllerSpeed(20.),
            crouch: ControllerSpeed(5.),
            slide: ControllerSpeed(25.),
        };
    }
}
