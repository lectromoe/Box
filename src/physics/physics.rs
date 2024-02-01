use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BoxyPhysicsPlugin;
impl Plugin for BoxyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}
